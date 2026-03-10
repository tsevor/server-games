use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};
use std::str;
use crate::games::game;
use std::thread;
use crate::game_lib::GameObjects;
use byteorder::{BigEndian, WriteBytesExt};

const HANDSHAKE_LEN: usize = 3;

fn accept_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut handshake = [0; HANDSHAKE_LEN];
    let _ = stream.read_exact(&mut handshake)?;
    //stream.set_nonblocking(true)?;

    if str::from_utf8(&handshake[..HANDSHAKE_LEN]) == Ok("hey") { 
        let _ = stream.write(b"sup"); 
    } else {
        println!("Connection closed");
        return Ok(());
    }

    game(stream);
    return Ok(());
}

pub fn start_listening() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3535")?;
    //listener.set_nonblocking(true)?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        let _ = accept_client(stream?);
    }
    //return stream;
    Ok(())
}

pub fn send_game_state(stream: &mut TcpStream, objects: &GameObjects) -> Result<(), Box<dyn std::error::Error>> {
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
/*
Packet structure:

*/