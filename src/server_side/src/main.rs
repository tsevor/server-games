


mod game_lib;

use game_lib::Rect;

fn main() {
    let test = Rect::new(10.0, 20.0, 100.0, 200.0);
    let (x, y) = test.pos();
    let (width, height) = test.size();
    println!("\n\n\nRect position: ({}, {}), size: ({}, {})\n\n", x, y, width, height);
    
}