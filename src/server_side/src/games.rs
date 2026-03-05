use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use byteorder::{BigEndian, WriteBytesExt};
use crate::game_lib::*;
use crate::input::Keys;

/*fn loadGame(

)*/

pub fn test_game() {
    
}

fn send_game_state(stream: &mut TcpStream, objects: &GameObjects) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize game state and send to client
    let mut data: Vec<u16> = Vec::new();

    if objects.rects().len() > 0{
        data.push(4); // Sends rects
        data.push(objects.rects().len() as u16);
        data.push(1); // Set to red to 0
        data.push(1); // Set to green to 0
        data.push(255); // Set blue to 255

        for rect in objects.rects() {
            data.push(rect.x);
            data.push(rect.y);
            data.push(rect.width);
            data.push(rect.height);
        }
    }

    if objects.circles().len() > 0{

        data.push(4); // Sends circles
        data.push(objects.circles().len() as u16);
        data.push(1); // Set to red to 0
        data.push(1); // Set to green to 0
        data.push(255); // Set blue to 255

        for circle in objects.circles() {
            data.push(circle.x);
            data.push(circle.y);
            data.push(circle.width);
            data.push(circle.height);
        }
    }   

    println!("Sending: {:?}",data);
    let mut bytes = Vec::with_capacity(data.len() * 2);
    for n in data {
        bytes.write_u16::<BigEndian>(n)? // network byte order
    }
    println!("Sending bytes: {:?}",bytes);
    stream.write_all(&bytes);
    Ok(())
}

pub fn game(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let _input = Arc::new(Mutex::new(Keys::default()));
    //to use it in other areas inport it and use method arc::clone(&input) to clone it and use it in other areas
    //and assign that to a variable

    let mut objects = GameObjects::new();

    let mut test = Rect::new(10, 20, 100, 200);
    objects.add_object(ObjectTypes::Rect(test));

    send_game_state(&mut stream, &objects);
    
    loop {
        let mut buffer = [0; 1028];
        let bytes_read = stream.read(&mut buffer)?;
        
        if bytes_read == 0 {
            // Connection closed by the peer
            println!("Connection closed");
            break Ok(());
        }

        println!("Read {} bytes: {:?}", bytes_read, &buffer[..bytes_read])
        
    }
}