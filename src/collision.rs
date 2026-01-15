use std::ops::Add;
use std::ops::Sub;

#[derive(Copy, Clone)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct Line {
    pub point1: Vec2d,
    pub point2: Vec2d,  
}

impl Vec2d {
    pub fn new(xi: f32,yi: f32) -> Vec2d{
        Vec2d { x: xi, y: yi}
    }
    pub fn dot(self, other: &Vec2d) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl Sub<&Vec2d> for Vec2d {
    type Output = Vec2d;

    fn sub(self, other: &Vec2d) -> Vec2d {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<Vec2d> for &Vec2d {
    type Output = Vec2d;

    fn sub(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn sub(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add<&Vec2d> for Vec2d {
    type Output = Vec2d;

    fn add(self, other: &Vec2d) -> Vec2d {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Vec2d> for &Vec2d {
    type Output = Vec2d;

    fn add(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn add(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}



impl Line{
    pub fn new(p1: Vec2d, p2: Vec2d) -> Line{
        Line { point1: p1, point2: p2 }
    }
}

pub fn check_collision(line: &Line, norm: &Vec2d, center: &Vec2d, radius: f32) -> bool{
    let dis = center - line.point1;
    if dis.dot(norm) - radius < 0.0 {true} else {false}
}

pub fn change_velocity_direction(line: &Line, norm: &Vec2d, velocity_pre: Vec2d) -> Vec2d{
    let tangent = line.point2 - line.point1;
    let speed_normal: f32 = norm.dot(&velocity_pre);
    let speed_rangent: f32 = tangent.dot(&velocity_pre);
    
    Vec2d{
        x: speed_normal * norm.x + speed_rangent * norm.y,
        y: speed_normal * tangent.x + speed_rangent * tangent.y,
    }
}

pub fn change_velocity_magnitude(velocity_pre: Vec2d, acceleration: Vec2d) -> Vec2d{
    velocity_pre + acceleration
}