mod games;
mod game_lib;
mod input;
mod tcp_socket;
use tcp_socket::start_listening;
use std::sync::{Arc, Mutex};



fn main() {
    //input arc method (arc is a wayt to share data between threads mutex makes it mutable) 
    let _ = start_listening();
    game(stream)
    //connection stuff getting mew clien=mts ect
    //assigm mew thread to cliemt with the ip an
}

//server thread code
//fn thread 