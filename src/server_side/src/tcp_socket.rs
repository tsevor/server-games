use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};
use std::error::Error;
use std::str;



fn accept_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    stream.set_nonblocking(true)?;

    if str::from_utf8(&buffer[..bytes_read]) == Ok("sup") { 
        let _ = stream.write(b"hey bud"); 
    } else {
        println!("Connection closed");
        return Ok(());
    }

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        
        if bytes_read == 0 {
            // Connection closed by the peer
            println!("Connection closed");
            break Ok(());
        }
        
        if buffer[0] == 4 {
            if bytes_read < 8 || (bytes_read - 4)%4 != 0 { let _ = stream.write(b"hey bud");  }
        }

        

        println!("Read {} bytes: {:?}", bytes_read, &buffer[..bytes_read])
        
    }
}

pub fn start_listening() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3535")?;
    //listener.set_nonblocking(true)?;

    // accept connections and process them serially
    for mut stream in listener.incoming() {
        let _ = accept_client(stream?);
    }
    return stream;
}