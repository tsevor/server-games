use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

static ID_COUNTER: AtomicU32 = AtomicU32::new(1);

fn next_id() -> u32 {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone)]
pub struct Transform {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub rgb: (u8,u8,u8),
}

impl Transform {
    pub fn new(x: u16, y: u16, width: u16, height: u16, rgb: (u8,u8,u8)) -> Self {
        Self { x, y, width, height, rgb }
    }

    pub fn move_by(&mut self, dx: i16, dy: i16) {
        if dx >= 0 {
            self.x += dx as u16;
        } else {
            self.x -= (-dx) as u16;
        }

        if dy >= 0 {
            self.y += dy as u16;
        } else {
            self.y -= (-dy) as u16;
        }
    }

    pub fn set_pos(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    pub fn pos(&self) -> (u16, u16) {
        (self.x, self.y)
    }// This is a placeholder. In a real implementation, you'd store this in the GameWorld struct.

    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
    pub fn color(&self) -> (u8,u8,u8){
        self.rgb
    }
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub transform: Transform,
}

#[derive(Debug, Clone)]
pub struct Circle {
    pub transform: Transform,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub transform: Transform,
    pub src: String,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub points: Vec<(u16, u16)>,
}

#[derive(Debug, Clone)]
pub enum GameObject {
    Rect(Rect),
    Circle(Circle),
    Image(Image),
    Polygon(Polygon),
}

pub struct GameWorld {
    background_color: (u8, u8, u8),
    objects: HashMap<u32, GameObject>,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            background_color: (0, 0, 0),
            objects: HashMap::new(),
        }
    }

    //========================
    // World Settings
    //========================

    pub fn set_background_color(&mut self, r: u8, g: u8, b: u8) {
        self.background_color = (r, g, b);
    }

    pub fn background_color(&self) -> (u8, u8, u8) {
        self.background_color
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }


    // ========================
    // Object Creation
    // ========================

    pub fn create_rect(&mut self, x: u16, y: u16, w: u16, h: u16, rgb: (u8,u8,u8)) -> u32 {
        let id = next_id();

        let rect = Rect {
            transform: Transform::new(x, y, w, h, rgb),
        };

        self.objects.insert(id, GameObject::Rect(rect));
        id
    }

    pub fn create_circle(&mut self, x: u16, y: u16, w: u16, h: u16, rgb: (u8,u8,u8)) -> u32 {
        let id = next_id();

        let circle = Circle {
            transform: Transform::new(x, y, w, h, rgb),
        };

        self.objects.insert(id, GameObject::Circle(circle));
        id
    }

    pub fn create_image(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        src: String,
        rgb: (u8,u8,u8),
    ) -> u32 {
        let id = next_id();

        let image = Image {
            transform: Transform::new(x, y, w, h, rgb),
            src,
        };

        self.objects.insert(id, GameObject::Image(image));
        id
    }

    pub fn create_polygon(&mut self, points: Vec<(u16, u16)>) -> u32 {
        let id = next_id();

        let polygon = Polygon { points };

        self.objects.insert(id, GameObject::Polygon(polygon));
        id
    }

    // ========================
    // Object Access
    // ========================

    pub fn get(&self, id: u32) -> Option<&GameObject> {
        self.objects.get(&id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut GameObject> {
        self.objects.get_mut(&id)
    }

    pub fn all(&self) -> impl Iterator<Item = (&u32, &GameObject)> {
        self.objects.iter()
    }

    // ========================
    // Transform Manipulation
    // ========================

    pub fn move_object(&mut self, id: u32, dx: i16, dy: i16) {
        if let Some(obj) = self.objects.get_mut(&id) {
            match obj {
                GameObject::Rect(r) => r.transform.move_by(dx, dy),
                GameObject::Circle(c) => c.transform.move_by(dx, dy),
                GameObject::Image(i) => i.transform.move_by(dx, dy),
                GameObject::Polygon(p) => {
                    for point in &mut p.points {
                        if dx >= 0 {
                            point.0 += dx as u16;
                        } else {
                            point.0 -= (-dx) as u16;
                        }

                        if dy >= 0 {
                            point.1 += dy as u16;
                        } else {
                            point.1 -= (-dy) as u16;
                        }
                    }
                }
            }
        }
    }

    pub fn set_position(&mut self, id: u32, x: u16, y: u16) {
        if let Some(obj) = self.objects.get_mut(&id) {
            match obj {
                GameObject::Rect(r) => r.transform.set_pos(x, y),
                GameObject::Circle(c) => c.transform.set_pos(x, y),
                GameObject::Image(i) => i.transform.set_pos(x, y),
                GameObject::Polygon(p) => {
                    if let Some((cx, cy)) = p.points.first().copied() {
                        let dx: u16 = x - cx;
                        let dy: u16 = y - cy;

                        for point in &mut p.points {
                            point.0 += dx;
                            point.1 += dy;
                        }
                    }
                }
            }
        }
    }

    pub fn get_position(&mut self, id: u32, x: u16, y: u16) {
        if let Some(obj) = self.objects.get_mut(&id) {
            match obj {
                GameObject::Rect(r) => r.transform.set_pos(x, y),
                GameObject::Circle(c) => c.transform.set_pos(x, y),
                GameObject::Image(i) => i.transform.set_pos(x, y),
                GameObject::Polygon(p) => {
                    if let Some((cx, cy)) = p.points.first().copied() {
                        let dx: u16 = x - cx;
                        let dy: u16 = y - cy;

                        for point in &mut p.points {
                            point.0 += dx;
                            point.1 += dy;
                        }
                    }
                }
            }
        }
    }

    pub fn is_collided(&mut self, id: u32, id2: u32) -> bool{
        let a = get(id);
        let b - get(id2);

        let axp = a.x + a.width;
        let axn = a.x - a.width;
        let ayp = a.y + a.height;
        let ayn = a.y - a.height;

        let bxn = b.x - b.width;
        let bxp = b.x + b.width;
        let byn = b.y - b.height;
        let byp = b.y + b.height;

        // Check separation
        if axp < bxn {
            return false;
        }

        if axn > bxp {
            return false;
        }

        if ayp < byn {
            return false;
        }

        if ayn > byp {
            return false;
        }
    }
    fn resolve_collision(a: &mut Box, b: &Box) {
        // Edges of A
        let axp = a.x + a.width;
        let axn = a.x - a.width;
        let ayp = a.y + a.height;
        let ayn = a.y - a.height;

        // Edges of B
        let bxp = b.x + b.width;
        let bxn = b.x - b.width;
        let byp = b.y + b.height;
        let byn = b.y - b.height;

        // Check if colliding
        if axp < bxn || axn > bxp || ayp < byn || ayn > byp {
            return; // no collision
        }

        // Compute overlap on each axis
        let overlap_x = if a.x < b.x {
            axp - bxn
        } else {
            bxp - axn
        };

        let overlap_y = if a.y < b.y {
            ayp - byn
        } else {
            byp - ayn
        };

        // Resolve along smallest axis
        if overlap_x < overlap_y {
            // Resolve X
            if a.x < b.x {
                move_object(a,-overlap_x,0);
            } else {
                move_object(a,overlap_x,0);
                
            }
        } else {
            // Resolve Y
            if a.y < b.y {
                move_object(a,-overlap_y,0);
            } else {
                move_object(a,overlap_y,0);
                
            }
        }
    }
    

}