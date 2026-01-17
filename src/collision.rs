use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

impl Vec2d {
    pub fn new(xi: f32,yi: f32) -> Vec2d{
        Vec2d { x: xi, y: yi}
    }
    pub fn dot(self, other: &Vec2d) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn normalize(self) -> Vec2d {
        let length = (self.x * self.x + self.y * self.y).sqrt();
        Vec2d {
            x: self.x / length,
            y: self.y / length,
        }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
macro_rules! impl_binary_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<&Vec2d> for Vec2d {
            type Output = Vec2d;
            fn $method(self, other: &Vec2d) -> Vec2d {
                Vec2d { x: self.x $op other.x, y: self.y $op other.y }
            }
        }

        impl $trait<Vec2d> for &Vec2d {
            type Output = Vec2d;
            fn $method(self, other: Vec2d) -> Vec2d {
                Vec2d { x: self.x $op other.x, y: self.y $op other.y }
            }
        }

        impl $trait<Vec2d> for Vec2d {
            type Output = Vec2d;
            fn $method(self, other: Vec2d) -> Vec2d {
                Vec2d { x: self.x $op other.x, y: self.y $op other.y }
            }
        }

        impl $trait<&Vec2d> for &Vec2d {
            type Output = Vec2d;
            fn $method(self, other: &Vec2d) -> Vec2d {
                Vec2d { x: self.x $op other.x, y: self.y $op other.y }
            }
        }
    };
}

impl_binary_op!(Add, add, +);
impl_binary_op!(Sub, sub, -);

impl Mul<f32> for Vec2d {
    type Output = Vec2d;

    fn mul(self, other: f32) -> Vec2d {
        Vec2d {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

macro_rules! impl_int_binary_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait<&ScreenPoint> for ScreenPoint {
            type Output = ScreenPoint;
            fn $method(self, other: &ScreenPoint) -> ScreenPoint {
                ScreenPoint { 
                    x: self.x $op other.x, 
                    y: self.y $op other.y 
                }
            }
        }

        impl $trait<ScreenPoint> for &ScreenPoint {
            type Output = ScreenPoint;
            fn $method(self, other: ScreenPoint) -> ScreenPoint {
                ScreenPoint { 
                    x: self.x $op other.x, 
                    y: self.y $op other.y 
                }
            }
        }

        impl $trait<ScreenPoint> for ScreenPoint {
            type Output = ScreenPoint;
            fn $method(self, other: ScreenPoint) -> ScreenPoint {
                ScreenPoint { 
                    x: self.x $op other.x, 
                    y: self.y $op other.y 
                }
            }
        }

        impl $trait<&ScreenPoint> for &ScreenPoint {
            type Output = ScreenPoint;
            fn $method(self, other: &ScreenPoint) -> ScreenPoint {
                ScreenPoint { 
                    x: self.x $op other.x, 
                    y: self.y $op other.y 
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScreenPoint {
    pub x: i32,
    pub y: i32,
}

impl ScreenPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn dot(self, other: &Vec2d) -> f32 {
        self.x as f32 * other.x + self.y as f32 * other.y
    }

    pub fn length(self) -> f32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt()
    }

    pub fn normalize(self) -> Vec2d {
        let length = ((self.x * self.x + self.y * self.y) as f32).sqrt();
        Vec2d {
            x: self.x as f32 / length,
            y: self.y as f32 / length,
        }
    }
}

// 应用宏到各种运算
impl_int_binary_op!(Add, add, +);
impl_int_binary_op!(Sub, sub, -);



#[derive(Copy, Clone)]
pub struct ScreenLine{
    pub point1:ScreenPoint,
    pub point2:ScreenPoint,
}


#[derive(Clone)]
pub struct CollisionLine {
    pub line: ScreenLine,
    pub normal: Vec2d
}

pub struct Boundaries {
    pub lines: Vec<CollisionLine>,
    pub damping_factor: f32,
}

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

pub fn check_collision(line: &ScreenLine, norm: &Vec2d, center: &ScreenPoint, radius: f32) -> bool{
    let dis = center - line.point1;
    if dis.dot(norm) - radius < 0.0 {true} else {false}
}

pub fn check_ball_ball_collision(center1: &ScreenPoint, center2: &ScreenPoint, radius1: f32, radius2: f32) -> bool{
    let dis_vec = center2 - center1;
    if dis_vec.length() < radius1 + radius2 {true} else {false}
}

pub fn change_velocity_direction(normal: &Vec2d, velocity: Vec2d) -> Vec2d {
    let dot_product = velocity.dot(normal);
    
    Vec2d {
        x: velocity.x - 2.0 * dot_product * normal.x,
        y: velocity.y - 2.0 * dot_product * normal.y,
    }
}

pub fn change_velocity_magnitude(velocity_pre: Vec2d, acceleration: Vec2d, dt: f32) -> Vec2d{
    velocity_pre + (acceleration * dt)
}