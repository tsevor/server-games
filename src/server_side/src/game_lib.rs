use std::sync::atomic::{AtomicU16, Ordering};

static ID_COUNTER: AtomicU16 = AtomicU16::new(0);

// data structures for game objects and packets
pub struct Rect {
    pub id: u16,
    pub x:u16,
    pub y:u16,
    pub width:u16,
    pub height:u16,
}
pub struct Circle {
    pub id: u16,
    pub x:u16,
    pub y:u16,
    pub width:u16,
    pub height:u16,
}
pub struct Image {
    pub id: u16,
    pub x:u16,
    pub y:u16,
    pub width:u16,
    pub height:u16,
    pub src: String,
}
pub struct Polygon {
    pub id: u16,
    points: Vec<(u16,u16)>,
}

pub struct GameObjects {
    rects: Vec<Rect>,
    circles: Vec<Circle>,
    images: Vec<Image>,
    polygons: Vec<Polygon>,
}

pub enum ObjectTypes {
    Rect(Rect),
    Circle(Circle),
    Image(Image),
    Polygon(Polygon),
}

// impl methods for game objects

impl Rect {
    pub fn new(x:u16, y:u16, width:u16, height:u16) -> Self{
        ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        Self { id: ID_COUNTER.load(Ordering::SeqCst), x, y, width, height }
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
    pub fn new(x:u16, y:u16, width:u16, height:u16) -> Self {
        ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        Circle { id: ID_COUNTER.load(Ordering::SeqCst), x, y, width, height }
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

impl Image {
    fn new(x:u16, y:u16, width:u16, height:u16, src:String) -> Self {
        ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        Image { id: ID_COUNTER.load(Ordering::SeqCst), x, y, width, height, src }
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

impl Polygon {
    fn new(points: Vec<(u16,u16)>) -> Self {
        ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        Polygon { id: ID_COUNTER.load(Ordering::SeqCst), points }
    }
    pub fn move_by(&mut self, dx:u16, dy:u16) {
        for point in &mut self.points {
            point.0 += dx;
            point.1 += dy;
        }
    }
    pub fn set_pos(&mut self , x:u16, y:u16){
        let (current_x, current_y) = self.pos();
        let dx = x - current_x;
        let dy = y - current_y;
        self.move_by(dx, dy);
    }
    pub fn pos(&self) -> (u16,u16) {
        if let Some(first_point) = self.points.first() {
            *first_point
        } else {
            (0, 0)
        }
    }
    pub fn size(&self) -> usize {
        self.points.len()
    }
}

impl GameObjects {
    pub fn new() -> Self {
        GameObjects {
            rects: Vec::new(),
            circles: Vec::new(),
            images: Vec::new(),
            polygons: Vec::new(),
        }
    }
    pub fn add_object(&mut self, obj: ObjectTypes) {
        match obj {
            ObjectTypes::Rect(rect) => self.rects.push(rect),
            ObjectTypes::Circle(circle) => self.circles.push(circle),
            ObjectTypes::Image(image) => self.images.push(image),
            ObjectTypes::Polygon(polygon) => self.polygons.push(polygon),
        }
    }   
    pub fn rects(&self) -> &[Rect] {
        &self.rects
    }
    pub fn circles(&self) -> &[Circle] {
        &self.circles
    }
}
