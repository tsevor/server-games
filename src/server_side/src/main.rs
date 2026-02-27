


mod game_lib;
mod input;
mod tcp_socket;
use tcp_socket::start_listening;
use std::sync::{Arc, Mutex};
use game_lib::Rect;
use input::Keys;



fn main() {
    //input arc method (arc is a wayt to share data between threads mutex makes it mutable) 
    let input = Arc::new(Mutex::new(Keys::default()));
    //to use it in other areas inport it and use method arc::clone(&input) to clone it and use it in other areas
    //and assign that to a variable

    let test = Rect::new(10, 20, 100, 200);
    let (x, y) = test.pos();
    let (width, height) = test.size();
    println!("\n\n\nRect position: ({}, {}), size: ({}, {})\n\n", x, y, width, height);
    let _ = start_listening();
}