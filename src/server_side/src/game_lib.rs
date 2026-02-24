

// data structures for game objects and packets
pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}
struct Image {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    src: String,
}
struct Polygon {
    points: Vec<(f32, f32)>,
}


// impl methods for game objects

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self{
        Self { x, y, width, height }
    }
    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
    pub fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    pub fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}


impl Circle {
    fn new(x: f32, y: f32, radius: f32) -> Self {
        Circle { x, y, radius }
    }
}