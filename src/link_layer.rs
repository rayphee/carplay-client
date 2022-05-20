//! # Link Layer for CarPlay Client
//! 
//! Mediates communication between the AutoBox Server hardware and the client
//! application.

use std::vec::Vec;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use std::thread;

extern crate rusb;
use rusb::*;

mod box_protocol;
use box_protocol::*;

const ID_VENDOR:u16 = 0x1314;
const ID_PRODUCT:u16 = 0x1520;

pub struct LinkLayer {
    device_handle: DeviceHandle<Context>,
    ep_in: u8,
    ep_out: u8,
    input_layer_rx: Receiver<()>,
    player_layer_tx: Sender<()>,
}

// TODO: Implement hotplug functionality
impl LinkLayer {
    pub fn new(player_layer_tx: Sender<()>, input_layer_rx: Receiver<()>) -> Self {
        let device = LinkLayer::get_dev().unwrap();
        let mut device_handle = device.open().unwrap();

        device_handle.reset().unwrap();
        device_handle.set_active_configuration(1).unwrap(); // Config 1 is the first valid one

        let config_desc = device_handle.device().config_descriptor(0).unwrap(); // Config _index_
        let iface_desc = config_desc.interfaces()
                                        .next().unwrap().descriptors()
                                        .next().unwrap();

        match device_handle.kernel_driver_active(iface_desc.interface_number()) {
            Ok(true) => device_handle.detach_kernel_driver(iface_desc.interface_number()).unwrap(),
            _ => {},
        }
        device_handle.claim_interface(iface_desc.interface_number()).unwrap();

        let (ep_in, ep_out) = LinkLayer::get_endpoints(&iface_desc).unwrap();

        device_handle.clear_halt(ep_in.address()).unwrap();
        device_handle.clear_halt(ep_out.address()).unwrap();

        println!("Claimed Interface # {:#?}", iface_desc.interface_number());
        println!("In Endpoint: {:#?}", ep_in);
        println!("Out Endpoint: {:#?}", ep_out);

        Self {
            device_handle, 
            ep_in: ep_in.address(), 
            ep_out: ep_out.address(), 
            input_layer_rx, 
            player_layer_tx
        }   
    }
    fn get_dev() -> Result<Device<Context>> {
        let mut device_opt: Option<Device<Context>> = None;
        loop {
            match device_opt {
                Some(ref _device_opt) => break,
                None => {
                    let usb_ctx = Context::new().unwrap();
                    for device in usb_ctx.devices().unwrap().iter() {
                        let device_desc = device.device_descriptor().unwrap();
                        if (device_desc.vendor_id() == ID_VENDOR) & 
                            (device_desc.product_id() == ID_PRODUCT) {
                            device_opt = Some(device);
                        }
                    }
                }
            }
        }
        Ok(device_opt.unwrap())
    }
    fn get_endpoints<'a>(iface_desc: &'a InterfaceDescriptor) -> 
                Result<(EndpointDescriptor<'a>, EndpointDescriptor<'a>)> {
        let mut endpoint_out: Option<EndpointDescriptor> = None;
        let mut endpoint_in: Option<EndpointDescriptor> = None;
        for endpoint_desc in iface_desc.endpoint_descriptors() {
            match endpoint_desc.direction() {
                Direction::Out => endpoint_out = Some(endpoint_desc),
                Direction::In => endpoint_in = Some(endpoint_desc),
            }
        }
        let ep_out = endpoint_out.unwrap();
        let ep_in = endpoint_in.unwrap();
        Ok((ep_in, ep_out))
    }

    fn rx_packet(&mut self) -> Result<MsgType> {
        let mut header_buf: [u8; 16] = [0; 16];

        // (1) We should not unwrap here, we need to handle potential errors
        self.device_handle.read_bulk(
            self.ep_in, 
            &mut header_buf,
            Duration::new(1,0)
        ).unwrap();
        let header: BoxMsgHeader = bincode::deserialize(&header_buf).unwrap();

        // Maybe this should be in a separate promote packet function?
        let mut payload_buf: Vec<u8> = Vec::with_capacity(header.msg_len as usize);
        // Same as (1)
        self.device_handle.read_bulk(
            self.ep_in, 
            &mut payload_buf,
            Duration::new(1,0)
        ).unwrap();
        let mut buf = header_buf.to_vec();
        buf.append(&mut payload_buf);

        // There should be a check that the packet matches the msg type we can id
        Ok(MsgType::deserialize(header.msg_type, &buf))
    }

    fn tx_packet<'de, T: BaseBoxMsg<'de>>(&mut self, packet: T) -> Result<usize> {
        println!("{:#?}", &packet.serialize());
        // Ok(8 as usize)
        self.device_handle.write_bulk(
            self.ep_out, 
            &packet.serialize(), // !! This is a vector !!
            Duration::new(1,0)
        )
    }

    fn tx_n_packets<'de, T: BaseBoxMsg<'de>>(&mut self, packets: Vec<Box<T>>) -> Result<usize> {
        let num_packets = packets.len();
        for packet in packets {
            match self.tx_packet(*packet) {
                Ok(_val) => continue,
                Err(e) => return Err(e)
            }
        }
        Ok(num_packets)
    }

    // fn tx_file(&mut self, filename: String) -> Result<usize> {
    // }

    // fn tx_append_file<T>(&mut self, filename: String, data: T) -> Result<usize> {
    // }

    pub fn start_box(&mut self) {
        let mut packet_vector: Vec<Box<MsgType>> = Vec::new();
        let video_data: Vec<u8> = Vec::new();
        let heartbeat_packet = MsgType::Heartbeat(Heartbeat::new()); 
        let open_packet = MsgType::OpenBox(OpenBox::new(1920, 720, 60));

        packet_vector.push(Box::new(heartbeat_packet));
        packet_vector.push(Box::new(open_packet));

        // Bullshit packets so we don't have annoying warnings for now
        packet_vector.push(Box::new(MsgType::DevPlug(DevPlug::new())));
        packet_vector.push(Box::new(MsgType::DevUnplug(DevUnplug::new())));
        packet_vector.push(Box::new(MsgType::Touch(Touch::new())));
        packet_vector.push(Box::new(MsgType::Video(Video::new(video_data))));
        packet_vector.push(Box::new(MsgType::Audio(Audio::new())));
        packet_vector.push(Box::new(MsgType::ButtonCtl(ButtonCtl::new())));
        packet_vector.push(Box::new(MsgType::BtAddr(BtAddr::new())));
        packet_vector.push(Box::new(MsgType::BtPin(BtPin::new())));
        packet_vector.push(Box::new(MsgType::ManInfo(ManInfo::new(-1, -1))));
        packet_vector.push(Box::new(MsgType::MultiTouch(MultiTouch::new())));
        packet_vector.push(Box::new(MsgType::SendFile(SendFile::new())));
        packet_vector.push(Box::new(MsgType::SwVer(SwVer::new())));

        self.tx_n_packets(packet_vector).unwrap();
    }
    
    // TODO: Rather than have a separate thread for heartbeat and send message,
    // it may be more efficient to queue messages and only send a heartbeat
    // when there are no messages to send and a timeout has been reached. This
    // is highly dependent on hardware requirements, please double check
    // feasibility. To accomplish this, there needs to be some degree of 
    // message coordination.
    // 
    // TODO: Perhaps implement message sending in an asynchronous fashion with
    // the associated timeout to prevent premature device disconnection. Do so
    // only if it makes sense for an asynchronous architecture here.
    pub fn communicate(&mut self) {
        println!("Connected");
        loop {
            let heartbeat_packet = MsgType::Heartbeat(Heartbeat::new()); // Needs to be tested thoroughly before true implementation
            self.tx_packet(heartbeat_packet).unwrap();
            let rx_packet = self.rx_packet();
            rx_packet.unwrap();
            // This block can be used to send data across the modules
            match self.input_layer_rx.try_recv() {
                Ok(_) => continue,
                Err(_e) => {},
            }
            self.player_layer_tx.send(()).unwrap();

            thread::sleep(Duration::new(1,0));
        }
    }
}

pub fn link_thread(tx: Sender<()>, rx: Receiver<()>) -> std::thread::JoinHandle<()> {
    thread::spawn(move|| {
        let mut link_layer = LinkLayer::new(tx, rx);
        link_layer.start_box();
        link_layer.communicate();
    })
}