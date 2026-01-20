pub mod collision_object;
mod collision;
pub use collision::{ Vec2d, ScreenLine, ScreenPoint};
pub use collision::{check_collision, check_ball_ball_collision, change_velocity_direction, change_velocity_magnitude};