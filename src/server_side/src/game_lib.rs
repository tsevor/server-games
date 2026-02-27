

// data structures for game objects and packets
pub struct Rect {
    x:u16,
    y:u16,
    width:u16,
    height:u16,
}
struct Circle {
    x:u16,
    y:u16,
    radius:u16,
}
struct Image {
    x:u16,
    y:u16,
    width:u16,
    height:u16,
    src: String,
}
struct Polygon {
    points: Vec<(u16,u16)>,
}


// impl methods for game objects

impl Rect {
    pub fn new(x:u16, y:u16, width:u16, height:u16) -> Self{
        Self { x, y, width, height }
    }
    pub fn move_by(&mut self, dx:u16, dy:u16) {
        self.x += dx;
        self.y += dy;
    }
    pub fn set_pos(&mut self , x:u16, y:u16){
        self.x = x;
        self.y = y;
    }
    pub fn pos(&self) -> (u16,u16) {
        (self.x, self.y)
    }
    pub fn size(&self) -> (u16,u16) {
        (self.width, self.height)
    }
}


impl Circle {
    fn new(x:u16, y:u16, radius:u16) -> Self {
        Circle { x, y, radius }
    }
    pub fn move_by(&mut self, dx:u16, dy:u16) {
        self.x += dx;
        self.y += dy;
    }
    pub fn set_pos(&mut self , x:u16, y:u16){
        self.x = x;
        self.y = y;
    }
    pub fn pos(&self) -> (u16,u16) {
        (self.x, self.y)
    }
    pub fn size(&self) -> u16 {
        self.radius
    }
} 