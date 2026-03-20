use std::net::TcpStream;
use std::io::{Read};
use std::sync::{Arc, Mutex};
use crate::game_lib::*;
use crate::input::Keys;
use crate::tcp_socket::send_game_state;
use crate::tcp_socket::get_input;
use std::time::Instant;
use std::thread::sleep;
use std::time::Duration;


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
    let player_id = world.create_rect(10, 20, 50, 50, (255,0,0));
    let circle_id = world.create_rect(100, 100, 40, 40,(0,255,0));

    world.move_object(player_id, 5, 5);
    world.set_position(circle_id, 200, 200);
    // let mut test2 = Circle::new(100, 50, 20, 40);
    // objects.add_object(ObjectTypes::Circle(test2));
    loop {
        let frame_start = Instant::now();
        let mut input = get_input(stream);
        world.move_object(player_id,  5*(input.key_d as i16 - input.key_a as i16), 5*(input.key_s as i16 - input.key_w as i16));
        world.resolve_collision(player_id, circle_id);
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
        
        let duration = frame_start.elapsed();
        let remaining = Duration::from_millis(10).saturating_sub(duration);
        std::thread::sleep(remaining);
        
    }
}

