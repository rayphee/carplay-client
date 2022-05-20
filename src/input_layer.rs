//! # Input Layer for CarPlay Client
//! 
//! Handles touch input for devices equipped with a libinput compatible digitizer 
//! and "keyboard" input for rotary dials, buttons, and other non touch control
//! surfaces. Potential plans to include microphone input for siri and phone calls.

use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::{RawFd, FromRawFd, IntoRawFd}};
use std::path::Path;
use input::{Libinput, LibinputInterface};
use std::sync::mpsc::{Sender};
use std::thread;

extern crate libc;
use libc::{O_RDONLY, O_RDWR, O_WRONLY};

pub struct InputLayer {
    input_ctx: Libinput,
    // link_layer_handle: bool, // Replace with handle type
}
struct InputInterface; 

// TODO: Limit scope of TouchInterface to touch events; abstract enough for useful
// function

/*  The following is from the reference example code in the input crate
    If it doesn't change from now until when the code is mature, well, it's a hell
    of an example */
impl LibinputInterface for InputInterface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into_raw_fd())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: RawFd) {
        unsafe {
            File::from_raw_fd(fd);
        }
    }
}

impl InputLayer {
    pub fn new() -> Self {
        let mut input_ctx = Libinput::new_with_udev(InputInterface);
        input_ctx.udev_assign_seat("seat0").unwrap();
        Self {input_ctx}
    }
    // pub fn dispatch(&mut self) -> Result<(), std::io::Error> {
    //     self.input_ctx.dispatch()
    // }
}

pub fn input_thread(_tx: Sender<()>) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut input_layer = InputLayer::new();
        loop {
            input_layer.input_ctx.dispatch().unwrap();
            for _event in &mut input_layer.input_ctx {
                
            }
        }
    })
}