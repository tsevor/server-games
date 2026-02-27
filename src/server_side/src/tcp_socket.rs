use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};
use std::str;
use crate::games::game;
use std::thread;

const HANDSHAKE_LEN: usize = 3;

fn accept_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut handshake = [0; HANDSHAKE_LEN];
    let _ = stream.read_exact(&mut handshake)?;
    //stream.set_nonblocking(true)?;

    if str::from_utf8(&handshake[..HANDSHAKE_LEN]) == Ok("sup") { 
        let _ = stream.write(b"hey bud\n"); 
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


/*
Packet structure:

*/