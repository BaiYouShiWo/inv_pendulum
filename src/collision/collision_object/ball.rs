use crate::collision::{ScreenPoint, Vec2d};
pub struct Ball {
    pub position: ScreenPoint,
    pub velocity: Vec2d, 
    pub radius: f32
}
impl Ball {
    pub fn update_position(&mut self, dt: f32) {
        self.position.x += (self.velocity.x * dt * 250.0) as i32;
        self.position.y += (self.velocity.y * dt * 250.0) as i32;
    }
}

pub struct Balls {
    pub balls: Vec<Ball>,
}