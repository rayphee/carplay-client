use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

mod input_layer;
mod link_layer;
mod player_layer;

// use crate::input_layer as imported_input_layer;
// use crate::link_layer as imported_link_layer;

fn main() {
    let (tx_input, rx_input) = mpsc::channel();
    let (tx_player, rx_player) = mpsc::channel();
    let input_thread_handle = input_layer::input_thread(tx_input);
    let _link_thread_handle = link_layer::link_thread(tx_player, rx_input);
    let _player_thread_handle = player_layer::player_thread(rx_player);

    // let link_object = imported_link_layer::LinkLayer::new(tx_player, rx_input);
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    input_thread_handle.join().unwrap(); // Stays for now; we need to find a better way
    // to wait on other threads (mainly more logical)
}