use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

static ID_COUNTER: AtomicU32 = AtomicU32::new(1);

fn next_id() -> u32 {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone)]
pub struct Transform {
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
    pub rgb: (u8,u8,u8),
}

impl Transform {
    pub fn new(x: i16, y: i16, width: i16, height: i16, rgb: (u8,u8,u8)) -> Self {
        Self { x, y, width, height, rgb }
    }

    pub fn move_by(&mut self, dx: i16, dy: i16) {
        self.x = self.x.saturating_add(dx);
        self.y = self.y.saturating_add(dy);
    }

    pub fn set_pos(&mut self, x: i16, y: i16) {
        self.x = x;
        self.y = y;
    }

    pub fn pos(&self) -> (i16, i16) {
        (self.x, self.y)
    }// This is a placeholder. In a real implementation, you'd store this in the GameWorld struct.

    pub fn size(&self) -> (i16, i16) {
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
    pub points: Vec<(i16, i16)>,
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

    pub fn create_rect(&mut self, x: i16, y: i16, w: i16, h: i16, rgb: (u8,u8,u8)) -> u32 {
        let id = next_id();

        let rect = Rect {
            transform: Transform::new(x, y, w, h, rgb),
        };

        self.objects.insert(id, GameObject::Rect(rect));
        id
    }

    pub fn create_circle(&mut self, x: i16, y: i16, w: i16, h: i16, rgb: (u8,u8,u8)) -> u32 {
        let id = next_id();

        let circle = Circle {
            transform: Transform::new(x, y, w, h, rgb),
        };

        self.objects.insert(id, GameObject::Circle(circle));
        id
    }

    pub fn create_image(
        &mut self,
        x: i16,
        y: i16,
        w: i16,
        h: i16,
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

    pub fn create_polygon(&mut self, points: Vec<(i16, i16)>) -> u32 {
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
                        point.0 += dx;
                        point.1 += dy;
                    }
                }
            }
        }
    }

    pub fn set_position(&mut self, id: u32, x: i16, y: i16) {
        if let Some(obj) = self.objects.get_mut(&id) {
            match obj {
                GameObject::Rect(r) => r.transform.set_pos(x, y),
                GameObject::Circle(c) => c.transform.set_pos(x, y),
                GameObject::Image(i) => i.transform.set_pos(x, y),
                GameObject::Polygon(p) => {
                    if let Some((cx, cy)) = p.points.first().copied() {
                        let dx: i16 = x - cx;
                        let dy: i16 = y - cy;

                        for point in &mut p.points {
                            point.0 += dx;
                            point.1 += dy;
                        }
                    }
                }
            }
        }
    }

    pub fn get_position(&mut self, id: u32) -> (i16, i16) {
        if let Some(obj) = self.objects.get_mut(&id) {
            match obj {
                GameObject::Rect(r) => r.transform.pos(),
                GameObject::Circle(c) => c.transform.pos(),
                GameObject::Image(i) => i.transform.pos(),
                GameObject::Polygon(p) => {
                    if let Some((cx, cy)) = p.points.first().copied() {
                        (cx, cy)
                    } else {
                        (0, 0)
                    }
                }
            }
        } else {
            (0, 0)
        }
    }

    pub fn get_size(&mut self, id: u32) -> (i16, i16) {
        if let Some(obj) = self.objects.get_mut(&id) {
            match obj {
                GameObject::Rect(r) => r.transform.size(),
                GameObject::Circle(c) => c.transform.size(),
                GameObject::Image(i) => i.transform.size(),
                GameObject::Polygon(p) => {
                    if let Some((cx, cy)) = p.points.first().copied() {
                        (cx, cy)
                    } else {
                        (0, 0)
                    }
                }
            }
        } else {
            (0, 0)
        }
    }

    pub fn is_collided(&mut self, id: u32, id2: u32) -> bool {
        // let a = self.get(id);
        // let b = self.get(id2);
        let (ax, ay) = self.get_position(id);
        let (aw, ah) = self.get_size(id);

        let axp = ax + aw;
        let axn = ax;
        let ayp = ay + ah;
        let ayn = ay - ah;

        let (bx, by) = self.get_position(id2);
        let (bw, bh) = self.get_size(id2);

        let bxn = bx;
        let bxp = bx + bw;
        let byn = by;
        let byp = by + bh;

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
        return true;
    }

    pub fn resolve_collision(&mut self, id: u32, id2: u32) -> bool {
        // let a = self.get(id);
        // let b = self.get(id2);

        // Edges of A
        let (ax, ay) = self.get_position(id);
        let (aw, ah) = self.get_size(id);

        let overcompensate = 2;

        let axp = ax + aw + overcompensate;
        let axn = ax + overcompensate; // - aw;
        let ayp = ay + ah + overcompensate;
        let ayn = ay + overcompensate; //- ah;

        let (bx, by) = self.get_position(id2);
        let (bw, bh) = self.get_size(id2);
        // Edges of B
        let bxp = bx + bw + overcompensate;
        let bxn = bx + overcompensate; //- bw;
        let byp = by + bh + overcompensate;
        let byn = by + overcompensate; //- bh;

        // Check if colliding
        if axp < bxn || axn > bxp || ayp < byn || ayn > byp {
            return false; // no collision
        }

        // Compute overlap on each axis
        let overlap_x = if ax < bx {
            axp - bxn
        } else {
            bxp - axn
        };

        let overlap_y = if ay < by {
            ayp - byn
        } else {
            byp - ayn
        };

        // Resolve along smallest axis
        // println!("X overlap: {} Y overlap: {}", overlap_x, overlap_y);
        if overlap_x < overlap_y {
            // Resolve X
            if ax < bx {
                self.move_object(id,-overlap_x,0);
            } else {
                self.move_object(id,overlap_x,0);

            }
        } else {
            // Resolve Y
            if ay < by {
                self.move_object(id,0,-overlap_y);
            } else {
                self.move_object(id,0,overlap_y);

            }
        }
        return true;
    }

}
pub fn f32toi16(float:f32) ->i16{
    float.round() as i16
}
