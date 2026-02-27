

mod games;
mod game_lib;
mod input;
mod tcp_socket;
use tcp_socket::start_listening;
use std::sync::{Arc, Mutex};
use game_lib::Rect;
use input::Keys;



fn main() {
    //input arc method (arc is a wayt to share data between threads mutex makes it mutable) 
    let _ = start_listening();
}