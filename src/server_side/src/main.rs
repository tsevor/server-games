mod games;
mod game_lib;
mod input;
mod tcp_socket;
use tcp_socket::start_listening;
use std::sync::{Arc, Mutex};



fn main() {
    //input arc method (arc is a wayt to share data between threads mutex makes it mutable)
    let quit = false;
    while(!quit){
        print!("\x1b[2J");
        print!("\x1b[48;5;67m");
        print!("\x1b[H");
        print!("\x1b[10E");
        print!("\x1b[");
        print!("                                       welcome to the server-games terminal for the server");
        print!("\x1b[10E]");
        print!("q:______________________________ quit");


        //keyboard input
        if event::poll(std::time::Duration::from_millis(10)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => {
                        quit = true;
                        break;
                    }
                    _ => {}
                }
            }
        }
        
    }
    print!("\x1b[0m]")
    let _ = start_listening();
    //connection stuff getting mew clien=mts ect
    //assigm mew thread to cliemt with the ip an
}

//server thread code
//fn thread 