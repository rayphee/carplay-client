//! # Box Protocol for CarPlay Client
//! 
//! Contains the message information required to exchange packets between
//! CarPlay client application and AutoBox Server hardware.

use std::vec::Vec;

extern crate serde;
extern crate bincode;
use serde::{Serialize, Deserialize};

const PROTOCOL_MAGIC:u32 = 0x55aa55aa;
const OPENBOX:u32 = 1;
const DEVPLUG:u32 = 2;
const DEVUNPLUG:u32 = 4;
const TOUCH:u32 = 5;
const VIDEO:u32 = 6;
const AUDIO:u32 = 7;
const BUTTONCTL:u32 = 8;
const BTADDR:u32 = 10;
const BTPIN:u32 = 12;
const MANINFO:u32 = 20;
const MULTITOUCH:u32 = 23; 
const SENDFILE:u32 = 153;
const HEARTBEAT:u32 = 170;
const SWVER:u32 = 204;

#[derive(Serialize, Deserialize)]
pub enum MsgType {
    OpenBox(OpenBox),
    DevPlug(DevPlug),
    DevUnplug(DevUnplug),
    Touch(Touch),
    Video(Video),
    Audio(Audio),
    ButtonCtl(ButtonCtl),
    BtAddr(BtAddr),
    BtPin(BtPin),
    ManInfo(ManInfo),
    MultiTouch(MultiTouch),
    SendFile(SendFile),
    Heartbeat(Heartbeat),
    SwVer(SwVer),
}

pub trait BaseBoxMsg<'de> {
    // Serialization is little endian
    fn serialize(&self) -> Vec<u8>; 
    fn deserialize(data_type: u32, data: &'de Vec<u8>) -> MsgType;
}

impl<'de> BaseBoxMsg<'de> for MsgType
where
    MsgType: Serialize + Deserialize<'de>,
    {
        fn serialize(&self) -> Vec<u8> {
            let mut ret: Vec<u8> = bincode::serialize(self).unwrap();
            ret.drain(..4); // discard enum id, this could be handled more elegantly
            ret
        }
        fn deserialize(data_type: u32, data: &'de Vec<u8>) -> MsgType {
            match data_type {
                // There should be a better way to do this; macros???
                OPENBOX => {
                    let inner: OpenBox = bincode::deserialize(data).unwrap();
                    MsgType::OpenBox(inner)
                }
                DEVPLUG => {
                    let inner: DevPlug = bincode::deserialize(data).unwrap();
                    MsgType::DevPlug(inner)
                }
                DEVUNPLUG => { 
                    let inner: DevUnplug = bincode::deserialize(data).unwrap();
                    MsgType::DevUnplug(inner)
                }
                TOUCH => { 
                    let inner: Touch = bincode::deserialize(data).unwrap();
                    MsgType::Touch(inner)
                }
                VIDEO => { 
                    let inner: Video = bincode::deserialize(data).unwrap();
                    MsgType::Video(inner)
                }
                AUDIO => { 
                    let inner: Audio = bincode::deserialize(data).unwrap();
                    MsgType::Audio(inner)
                }
                BUTTONCTL => { 
                    let inner: ButtonCtl = bincode::deserialize(data).unwrap();
                    MsgType::ButtonCtl(inner)
                }
                BTADDR => { 
                    let inner: BtAddr = bincode::deserialize(data).unwrap();
                    MsgType::BtAddr(inner)
                }
                BTPIN => { 
                    let inner: BtPin = bincode::deserialize(data).unwrap();
                    MsgType::BtPin(inner)
                }
                MANINFO => { 
                    let inner: ManInfo = bincode::deserialize(data).unwrap();
                    MsgType::ManInfo(inner)
                }
                MULTITOUCH => { 
                    let inner: MultiTouch = bincode::deserialize(data).unwrap();
                    MsgType::MultiTouch(inner)
                }
                SENDFILE => { 
                    let inner: SendFile = bincode::deserialize(data).unwrap();
                    MsgType::SendFile(inner)
                }
                HEARTBEAT => { 
                    let inner: Heartbeat = bincode::deserialize(data).unwrap();
                    MsgType::Heartbeat(inner)
                }
                SWVER => { 
                    let inner: SwVer = bincode::deserialize(data).unwrap();
                    MsgType::SwVer(inner)
                }
                _ => {
                    let inner: Heartbeat = bincode::deserialize(data).unwrap();
                    MsgType::Heartbeat(inner)
                }
            }
        }
    }

#[derive(Serialize, Deserialize)]
pub struct BoxMsgHeader {
    magic: u32,
    pub msg_len: u32, // in bytes
    pub msg_type: u32,
    msg_parity: u32,
}

impl BoxMsgHeader {
    pub fn new(msg_type: u32, msg_len: u32) -> Self {
        Self {
            magic: PROTOCOL_MAGIC,
            msg_len,
            msg_type,
            msg_parity: (msg_type as i32 ^ -1) as u32 & 0xffffffff
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OpenBox {
    header: BoxMsgHeader,
    width: u32,
    height: u32,
    framerate: u32,
    format:  u32,
    packet_max: u32,
    box_ver: u32,
    phone_work_mode: u32,
}

#[derive(Serialize, Deserialize)]
pub struct DevPlug{
    header: BoxMsgHeader,
}

#[derive(Serialize, Deserialize)]
pub struct DevUnplug{
    header: BoxMsgHeader,
}

#[derive(Serialize, Deserialize)]
pub struct Touch{
    header: BoxMsgHeader,
}

#[derive(Serialize, Deserialize)]
pub struct Video{
    header: BoxMsgHeader,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Audio{
    header: BoxMsgHeader,
}

#[derive(Serialize, Deserialize)]
pub struct ButtonCtl{
    header: BoxMsgHeader,
}

#[derive(Serialize, Deserialize)]
pub struct BtAddr{
    header: BoxMsgHeader,
}

#[derive(Serialize, Deserialize)]
pub struct BtPin{
    header: BoxMsgHeader,
}

#[derive(Serialize, Deserialize)]
pub struct ManInfo{
    header: BoxMsgHeader,
    brand: i32,
    model: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MultiTouch{
    header: BoxMsgHeader,
}

#[derive(Serialize, Deserialize)]
pub struct SendFile{
    header: BoxMsgHeader,
}

#[derive(Serialize, Deserialize)]
pub struct Heartbeat {
    header: BoxMsgHeader,
}

#[derive(Serialize, Deserialize)]
pub struct SwVer{
    header: BoxMsgHeader,
}

impl OpenBox {
    pub fn new(width: u32, height: u32, framerate: u32) -> Self {
        Self {
            header: BoxMsgHeader::new(OPENBOX, 28), // 28 bytes
            width, 
            height, 
            framerate, 
            format: 0x5, 
            packet_max: 49152, 
            box_ver: 2, 
            phone_work_mode: 2
        }
    }
}

impl DevPlug {
    pub fn new() -> Self {
        Self {
            header: BoxMsgHeader::new(DEVPLUG, 0), 
        }
    }
}

impl DevUnplug {
    pub fn new() -> Self {
        Self {
            header: BoxMsgHeader::new(DEVUNPLUG, 0), 
        }
    }
}

impl Touch {
    pub fn new() -> Self {
        Self {
            header: BoxMsgHeader::new(TOUCH, 0), 
        }
    }
}

impl Video {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            header: BoxMsgHeader::new(VIDEO, 0), 
            data
        }
    }
}

impl Audio {
    pub fn new() -> Self {
        Self {
            header: BoxMsgHeader::new(AUDIO, 0), 
        }
    }
}

impl ButtonCtl {
    pub fn new() -> Self {
        Self {
            header: BoxMsgHeader::new(BUTTONCTL, 0), 
        }
    }
}

impl BtAddr {
    pub fn new() -> Self {
        Self {
            header: BoxMsgHeader::new(BTADDR, 0), 
        }
    }
}

impl BtPin {
    pub fn new() -> Self {
        Self {
            header: BoxMsgHeader::new(BTPIN, 0), 
        }
    }
}

impl ManInfo {
    pub fn new(brand: i32, model: i32) -> Self {
        Self {
            header: BoxMsgHeader::new(MANINFO, 2), // Should be length of payload
            brand, 
            model,
        }
    }
}

impl MultiTouch {
    pub fn new() -> Self {
        Self {
            header: BoxMsgHeader::new(MULTITOUCH, 0), 
        }
    }
}

impl SendFile {
    pub fn new() -> Self {
        Self {
            header: BoxMsgHeader::new(SENDFILE, 0), 
        }
    }
}

impl Heartbeat {
    pub fn new() -> Self {
        Self {
            header: BoxMsgHeader::new(HEARTBEAT, 0), 
        }
    }
}

impl SwVer {
    pub fn new() -> Self {
        Self {
            header: BoxMsgHeader::new(SWVER, 0), 
        }
    }
}