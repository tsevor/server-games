use std::net::TcpStream;
use std::io::{Read};
// use std::sync::{Arc, Mutex};
use crate::game_lib::*;
use crate::tcp_socket::send_game_state;
use crate::tcp_socket::get_input;
use std::time::Instant;
use std::time::Duration;
use crate::terminal_lib;


/*fn loadGame(

)*/

// pub fn test_game(objects: GameObjects) ->GameObjects {
//     let mut objects = GameObjects::new();
//     return game_state
// }

pub fn win() {
    println!("Win")
}

pub fn game(mut stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
   
    //to use it in other areas inport it and use method arc::clone(&input) to clone it and use it in other areas
    //and assign that to a variable
    let mut world = GameWorld::new();

    world.set_background_color(0, 0, 255);
    let player_id = world.create_rect(10, 20, 50, 50, (255,0,0));
    let mut level1: Vec<u32> = Vec::new();
    level1.push(world.create_rect(300,500,730,50,(30,255,30)));
    level1.push(world.create_rect(750,300,15,1000,(30,255,30)));
    level1.push(world.create_rect(0,300,15,1000,(30,255,30)));
    level1.push(world.create_rect(300,0,730,50,(30,255,30)));
    level1.push(world.create_rect(0, 500, 500, 40,(0,255,0)));
    // level1.push(world.create_rect());
    let level1_start: (i16, i16) = (100,400);
    let level1_end: (i16, i16) = (600,300);
    let mut platform_ids: Vec<u32> = Vec::new();
    platform_ids.push(world.create_rect(300,500,720,10,(50,50,255)));


    
    let win_rect_id = world.create_rect(500, 400, 40, 40,(255,255,0));

    let mut player_yvel:f32 = 0_f32;
    // let mut test2 = Circle::new(100, 50, 20, 40);
    // objects.add_object(ObjectTypes::Circle(test2));

    //can we m
    loadlevel(&mut world, player_id, win_rect_id, &mut platform_ids, level1, level1_start, level1_end);
    loop {
        
        let frame_start = Instant::now();
        let input = get_input(stream);
        //collision stuff
        world.move_object(player_id,  5*(input.key_d as i16 - input.key_a as i16),f32toi16(player_yvel));
        //jump
        
        //change to load a different level pls??
        if world.is_collided(player_id, win_rect_id) {
            win();
        }
        let player_touching = resolve_platforms_collision(&mut world, player_id, &platform_ids);
        if player_touching{
            player_yvel = 0_f32;
            if input.key_w {
                player_yvel -= 10_f32;
            }
        }else{
            let (_player_x, player_y) = world.get_position(player_id);
            terminal_lib::add_window(10, 30, &player_y.to_string());
            terminal_lib::add_window(40, 30, &player_yvel.to_string());
            player_yvel +=0.2_f32;
        }
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
        let remaining = Duration::from_millis(10).saturating_sub(duration); // Fixed frame rate at 100 frames
        std::thread::sleep(remaining);
        
    }
}
//loop over platform ids and return if collided
fn is_platforms_collided(world: &mut GameWorld, operation :u32 , ids: &Vec<u32>) -> bool {
    for plat in ids{
        if world.is_collided(operation, *plat){
            return true;
        }
    }
    return false;
}
//loop over platform ids and resolve collisions
fn resolve_platforms_collision(world: &mut GameWorld, operation: u32, ids: &Vec<u32>) -> bool {
    let mut collided = false;
    for plat in ids {
        terminal_lib::add_window(50, 30, &plat.to_string());
        if world.resolve_collision(operation, *plat) { collided = true; } 
    }
    collided
}
//load level
fn loadlevel(world: &mut GameWorld, player_id: u32, win_rect_id: u32, platform_ids: &mut Vec<u32>, mut platforms: Vec<u32>, start: (i16, i16), end: (i16, i16)){
    platform_ids.clear();
    platform_ids.append(&mut platforms);
    world.set_position(player_id,start.0,start.1);
    world.set_position(win_rect_id,end.0,end.1);

}



