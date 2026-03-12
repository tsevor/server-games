use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use crate::game_lib::*;
use crate::input::Keys;
use crate::tcp_socket::send_game_state;

/*fn loadGame(

)*/

// pub fn test_game(objects: GameObjects) ->GameObjects {
//     let mut objects = GameObjects::new();
//     return game_state
// }



pub fn game(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let _input = Arc::new(Mutex::new(Keys::default()));
    //to use it in other areas inport it and use method arc::clone(&input) to clone it and use it in other areas
    //and assign that to a variable
    let mut objects = GameObjects::new();
    // let mut test = Rect::new(10, 20, 100, 200);
    objects.add_object("player",ObjectTypes::Rect(Rect::new(10, 20, 100, 200)));
    // let mut test2 = Circle::new(100, 50, 20, 40);
    // objects.add_object(ObjectTypes::Circle(test2));
    send_game_state(&mut stream, &objects);

    loop {
        let mut buffer = [0; 1028];
        let bytes_read = stream.read(&mut buffer)?;
        
        if bytes_read == 0 {
            // Connection closed by the peer
            println!("Connection closed");
            break Ok(());
        }

        // test.move_by(1,0);
        send_game_state(&mut stream, &objects);
        println!("Read {} bytes: {:?}", bytes_read, &buffer[..bytes_read])
    
    }
}

