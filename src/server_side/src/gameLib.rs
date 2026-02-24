

// data structures for game objects and packets
struct rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
struct circle {
    x: f32,
    y: f32,
    radius: f32,
}
struct image {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    src: String,
}
struct polygon {
    points: Vec<(f32, f32)>,
}


// impl methods for game objects

impl rect {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        rect { x, y, width, height }
    }
    fn move_by(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
    fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}


impl circle {}
    fn new{x: f32, y}
}