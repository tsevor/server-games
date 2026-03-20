use std::net::TcpStream;
use std::io::{Read};
use std::sync::{Arc, Mutex};
use crate::game_lib::*;
use crate::input::Keys;
use crate::tcp_socket::send_game_state;
use crate::tcp_socket::get_input;

/*fn loadGame(

)*/

// pub fn test_game(objects: GameObjects) ->GameObjects {
//     let mut objects = GameObjects::new();
//     return game_state
// }



pub fn game(mut stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
   
    //to use it in other areas inport it and use method arc::clone(&input) to clone it and use it in other areas
    //and assign that to a variable
    let mut world = GameWorld::new();

    world.set_background_color(0, 0, 255);
    let rect_id = world.create_rect(10, 20, 50, 50, (255,0,0));
    let circle_id = world.create_circle(100, 100, 40, 40,(0,255,0));

    world.move_object(rect_id, 5, 5);
    world.set_position(circle_id, 200, 200);
    // let mut test2 = Circle::new(100, 50, 20, 40);
    // objects.add_object(ObjectTypes::Circle(test2));
    loop {
        
        let mut input = get_input(stream);
        world.move_object(rect_id,  input.key_d - input.key_a, input.key_w - input.key_s);
        send_game_state(&mut stream, &world);
        
        let mut buffer = [0; 1028];
        let bytes_read = stream.read(&mut buffer)?;
        
        
        if bytes_read == 0 {
            // Connection closed by the peer
            println!("Connection closed");
            break Ok(());
        }
        match buffer[0] {
            1 => {
                // println!("normal byte");
            } // Normal code
            _ => {}
            // Put other codes here
        }


        // println!("Read {} bytes: {:?}", bytes_read, &buffer[..bytes_read]);
        std::thread::sleep(std::time::Duration::from_millis(10));
        
    }
}

