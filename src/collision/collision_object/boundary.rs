use crate::collision::{ScreenLine,Vec2d};

#[derive(Clone)]
pub struct CollisionLine {
    pub line: ScreenLine,
    pub normal: Vec2d
}

pub struct Boundaries {
    pub lines: Vec<CollisionLine>,
    pub damping_factor: f32,
}