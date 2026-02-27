

// data structures for game objects and packets
pub struct Rect {
    x:i32,
    y:i32,
    width:i32,
    height:i32,
}
struct Circle {
    x:i32,
    y:i32,
    radius:i32,
}
struct Image {
    x:i32,
    y:i32,
    width:i32,
    height:i32,
    src: String,
}
struct Polygon {
    points: Vec<(i32,i32)>,
}


// impl methods for game objects

impl Rect {
    pub fn new(x:i32, y:i32, width:i32, height:i32) -> Self{
        Self { x, y, width, height }
    }
    pub fn move_by(&mut self, dx:i32, dy:i32) {
        self.x += dx;
        self.y += dy;
    }
    pub fn set_pos(&mut self , x:i32, y:i32){
        self.x = x;
        self.y = y;
    }
    pub fn pos(&self) -> (i32,i32) {
        (self.x, self.y)
    }
    pub fn size(&self) -> (i32,i32) {
        (self.width, self.height)
    }
}


impl Circle {
    fn new(x:i32, y:i32, radius:i32) -> Self {
        Circle { x, y, radius }
    }
    pub fn move_by(&mut self, dx:i32, dy:i32) {
        self.x += dx;
        self.y += dy;
    }
    pub fn set_pos(&mut self , x:i32, y:i32){
        self.x = x;
        self.y = y;
    }
    pub fn pos(&self) -> (i32,i32) {
        (self.x, self.y)
    }
    pub fn size(&self) -> (i32,i32) {
        (self.radius * 2, self.radius * 2)
    }
} 