mod games;
mod game_lib;
mod input;
mod tcp_socket;
mod terminal_lib;
use tcp_socket::start_listening;
// use std::sync::{Arc, Mutex}; // Might need this later but removed to get rid of annoying warning
use std::env;
use std::process;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::thread;

use std::io::Write;





fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Format: {} <ip> <port>", args[0]);
        process::exit(0);
    }
    //input arc method (arc is a wayt to share data between threads mutex makes it mutable)
    enable_raw_mode().unwrap();
    let mut quit = false;
    terminal_ui();
    thread::spawn(move || {
        let _ = start_listening(&args[1],&args[2]);
    });
    while  !quit {

        print!("\x1b[48;5;67m");
        std::io::stdout().flush().unwrap();

        //keyboard input
        if event::poll(std::time::Duration::from_millis(10)).expect("POLL ERROR") {
            match event::read(){
                Ok (_event)=>{
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
                Err(e) => {
                eprint!("failed to read event: {}", e);
                }
            }

        }
    }
    std::io::stdout().flush().unwrap();
    print!("\x1b[2J");
    print!("\x1b[?25h");
    print!("\x1b[0m");
    print!("\x1b[1000E");


    disable_raw_mode().unwrap();
    //connection stuff getting mew clien=mts ect
    //assigm mew thread to cliemt with the ip an
}

//server thread code
//fn thread

fn terminal_ui(){
    enable_raw_mode().unwrap();
    print!("\x1b[?25l");
    print!("\x1b[2J");
    print!("\x1b[48;5;67m");
    print!("\x1b[H");
    for _n in 1..100{

        print!("\x1b[48;5;238m");
        print!("\x1b[K");
        print!("\x1b[1E");

    }
    print!("\x1b[48;5;67m");
    print!("\x1b[H");
    print!("\x1b[1E");
    print!("\x1b[10C");
    print!("                                                           ");
    print!("\x1b[1E");
    print!("\x1b[10C");
    print!("            welcome to the server-games terminal           █");
    print!("\x1b[1E");
    print!("\x1b[10C");
    print!("                                                           █");
    print!("\x1b[1E");
    print!("\x1b[10C");
    print!("\x1b[48;5;238m");
    print!(" ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀");
    print!("\x1b[48;5;67m");
    print!("\x1b[10E");
    print!("                                         ");
    print!("\x1b[1E");
    print!("  q:______________________________ quit  █");
    print!("\x1b[1E");
    print!("                                         █");
    print!("\x1b[1E");
    print!("\x1b[48;5;238m");
    print!(" ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀");
    print!("\x1b[48;5;67m");
    print!("\x1b[48;5;62m");

    std::io::stdout().flush().unwrap();
}
