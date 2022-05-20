//! # Player Layer for CarPlay Client
//! 
//! Decodes and displays CarPlay interface after receiving the appropriate 
//! serialized packets from the AutoBox Server hardware.

use std::sync::mpsc::Receiver;
use std::thread;

// extern crate mpv;
// use mpv::*;

// This module is empty for the time being; minimum feature validation
// needs to be performed before this module will be populated.

// pub struct PlayerLayer {
// }

pub fn player_thread(_rx: Receiver<()>) -> std::thread::JoinHandle<()> {
    thread::spawn(move|| {
    })
}