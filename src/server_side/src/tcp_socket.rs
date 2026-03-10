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
    let mut data: Vec<u8> = Vec::new();

    if objects.rects().len() > 0{
        data.push(4); // Sends rects
        data.push(objects.rects().len() as u8);
		// set color to blue
        data.push(0);
        data.push(0);
        data.push(255);

        for rect in objects.rects() {
            data.push((rect.x & 0xff) as u8);
			data.push((rect.x >> 8) as u8);
            data.push((rect.y & 0xff) as u8);
			data.push((rect.y >> 8) as u8);
            data.push((rect.width & 0xff) as u8);
			data.push((rect.width >> 8) as u8);
            data.push((rect.height & 0xff) as u8);
			data.push((rect.height >> 8) as u8);
        }
    }

    if objects.circles().len() > 0{

        data.push(6); // Sends circles
        data.push(objects.circles().len() as u8);
		// set color to green
        data.push(0);
        data.push(255);
        data.push(0);

        for circle in objects.circles() {
            data.push((circle.x & 0xff) as u8);
			data.push((circle.x >> 8) as u8);
            data.push((circle.y & 0xff) as u8);
			data.push((circle.y >> 8) as u8);
            data.push((circle.width & 0xff) as u8);
			data.push((circle.width >> 8) as u8);
            data.push((circle.height & 0xff) as u8);
			data.push((circle.height >> 8) as u8);
        }
    }

	data.push(0); // refresh screen

    println!("Sending: {:?}",data);
    let mut bytes = Vec::with_capacity(data.len() * 2);
    for n in data {
        bytes.write_u8(n)? // network byte order
    }
    println!("Sending bytes: {:?}",bytes);
	stream.write_all(&bytes)?;
    Ok(())
}
/*
Packet structure:

*/