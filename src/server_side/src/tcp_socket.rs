use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};
use std::str;
use crate::input::*;
use crate::games::game;
use std::thread;
use crate::game_lib::*;
use byteorder::{WriteBytesExt};
// use std::fmt::format; // Might need this later...

const HANDSHAKE_LEN: usize = 3;
const WINDOW_SIZE: (u16, u16) = (720, 540);

fn accept_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut handshake = [0; HANDSHAKE_LEN];
    let _ = stream.read_exact(&mut handshake)?;
    //stream.set_nonblocking(true)?;

    if str::from_utf8(&handshake[..HANDSHAKE_LEN]) == Ok("hey") { 
        let mut data: Vec<u8> = Vec::new();
        
        data.push((WINDOW_SIZE.0 & 0xff) as u8);
        data.push((WINDOW_SIZE.0 >> 8) as u8);
        data.push((WINDOW_SIZE.1 & 0xff) as u8);
        data.push((WINDOW_SIZE.1 >> 8) as u8);

        let mut bytes = Vec::with_capacity(data.len() * 2);
        bytes.extend_from_slice(b"sup");

        for n in data {
            bytes.write_u8(n)? // network byte order
        }
        let _ = stream.write(&bytes); 
    } else {
        println!("Connection closed");
        return Ok(());
    }

    game(stream);
    return Ok(());
}

pub fn start_listening(host: &str, port: &str) -> std::io::Result<()> {
    let listen_string = format!("{}:{}", host, port);

    let listener = TcpListener::bind(listen_string)?;
    //listener.set_nonblocking(true)?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        thread::spawn(move || {
            let _ = accept_client(stream.expect("REASON"));
        }
    );
    }
    //return stream;
    Ok(())
}

pub fn push_background_color(world: &GameWorld, data: &mut Vec<u8>) {
    let (r, g, b) = world.background_color();
    data.push(1); // background color packet
    data.push(r);
    data.push(g);
    data.push(b);
}

pub fn send_game_state(stream: &mut TcpStream, world: &GameWorld) -> Result<(), Box<dyn std::error::Error>> {
    // println!("Serializing game state...");
    // Serialize game state and send to client
    let mut data: Vec<u8> = Vec::new();
    push_background_color(world, &mut data);

    let mut rect_vec: Vec<u8> = Vec::new();
    rect_vec.push(4);
    let mut rect_count: u8 = 0;
    let mut circle_vec: Vec<u8> = Vec::new();
    let mut circle_count: u8 = 0;
    circle_vec.push(6);
    for (id, obj) in world.all() {
        match obj {
            GameObject::Rect(rect) => {
                rect_count += 1;
                // set color to blue
                
                let (r, g, b) = rect.transform.color();
                rect_vec.push(r);
                rect_vec.push(g);
                rect_vec.push(b);
                
                let (x, y) = rect.transform.pos(); // (x, y)
                let (width, height) = rect.transform.size(); // (width, height)
                rect_vec.push((x & 0xff) as u8);
                rect_vec.push((x >> 8) as u8);
                rect_vec.push((y & 0xff) as u8);
                rect_vec.push((y >> 8) as u8);
                rect_vec.push((width & 0xff) as u8);
                rect_vec.push((width >> 8) as u8);
                rect_vec.push((height & 0xff) as u8);
                rect_vec.push((height >> 8) as u8);
            }

            GameObject::Circle(circle) => {
                circle_count += 1;

                let (r, g, b) = circle.transform.color();
                circle_vec.push(r); // Add color :/
                circle_vec.push(g);
                circle_vec.push(b);

                let (x, y) = circle.transform.pos(); // (x, y)
                let (width, height) = circle.transform.size(); // (width, height)
                circle_vec.push((x & 0xff) as u8);
                circle_vec.push((x >> 8) as u8);
                circle_vec.push((y & 0xff) as u8);
                circle_vec.push((y >> 8) as u8);
                circle_vec.push((width & 0xff) as u8);
                circle_vec.push((width >> 8) as u8);
                circle_vec.push((height & 0xff) as u8);
                circle_vec.push((height >> 8) as u8);
            }

            GameObject::Image(image) => {
                println!("Image {} src {}", id, image.src);
            }

            GameObject::Polygon(poly) => {
                println!("Polygon {} with {} points", id, poly.points.len());
            }
        }

    }


    /*/ legacy code
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
    }*/

    rect_vec.insert(1, rect_count);
    circle_vec.insert(1, circle_count); // Insert length at the beginning
    if rect_count > 0 { data.append(&mut rect_vec); }
    
    if circle_count > 0 { data.append(&mut circle_vec); }
	data.push(0); // refresh screen
    let mut bytes = Vec::with_capacity(data.len() * 2);
    for n in data {
        bytes.write_u8(n)? // network byte order
    }
    // println!("Sending bytes: {:?}",bytes);
	stream.write_all(&bytes)?;
    Ok(())
}
pub fn send_game_data(){
    objects = copy::games::objects();
}

pub fn get_input(stream: &mut TcpStream) -> keys {
    // --- 1. Request keyboard state from client ---
    stream.write_all(&[0x80]).expect("Failed to send kb request");

    // --- 2. Read response packet header ---
    let mut header = [0u8; 2];
    stream.read_exact(&mut header).expect("Failed to read packet header");

    let packet_id = header[0];
    let key_count  = header[1]; // N = number of keys currently pressed

    let mut input = keys::default();

    // --- 3. Validate it's a keyboard state packet (0x03) ---
    if packet_id != 0x03 {
        eprintln!("Expected keyboard state packet (0x03), got ({:#04x})", packet_id);
        return input;
    }

    // --- 4. Read N key bytes ---
    let mut key_bytes = vec![0u8; key_count as usize];
    stream.read_exact(&mut key_bytes).expect("Failed to read key bytes");

    // --- 5. Match ASCII key codes to struct fields ---
    for k in key_bytes {
        match k {
            b'a' => input.a = true,
            b'b' => input.b = true,
            b'c' => input.c = true,
            b'd' => input.d = true,
            b'e' => input.e = true,
            b'f' => input.f = true,
            b'g' => input.g = true,
            b'h' => input.h = true,
            b'i' => input.i = true,
            b'j' => input.j = true,
            b'k' => input.k = true,
            b'l' => input.l = true,
            b'm' => input.m = true,
            b'n' => input.n = true,
            b'o' => input.o = true,
            b'p' => input.p = true,
            b'q' => input.q = true,
            b'r' => input.r = true,
            b's' => input.s = true,
            b't' => input.t = true,
            b'u' => input.u = true,
            b'v' => input.v = true,
            b'w' => input.w = true,
            b'x' => input.x = true,
            b'y' => input.y = true,
            b'z' => input.z = true,

            b' '  => input.space     = true,
            b'\n' => input.enter     = true,
            b'\x08' => input.backspace = true,
            b'\x1b' => input.esc      = true,

            _ => {} // unknown/unhandled key
        }
    }

    input
}


/*
Packet structure:

*/