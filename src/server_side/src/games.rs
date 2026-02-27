use std::net::TcpStream;
use std::io::Read;
use std::sync::{Arc, Mutex};
use crate::game_lib::Rect;
use crate::input::Keys;

/*fn loadGame(

)*/

pub fn test_game() {
    
}

pub fn game(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let _input = Arc::new(Mutex::new(Keys::default()));
    //to use it in other areas inport it and use method arc::clone(&input) to clone it and use it in other areas
    //and assign that to a variable

    let test = Rect::new(10, 20, 100, 200);
    let (x, y) = test.pos();
    let (width, height) = test.size();
    println!("\n\n\nRect position: ({}, {}), size: ({}, {})\n\n", x, y, width, height);
    
    loop {
        let mut buffer = [0; 1028];
        let bytes_read = stream.read(&mut buffer)?;
        
        /*if bytes_read == 0 {
            // Connection closed by the peer
            println!("Connection closed");
            break Ok(());
        }       */

        println!("Read {} bytes: {:?}", bytes_read, &buffer[..bytes_read])
        
    }
}