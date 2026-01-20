use crate::inverted_pendulum::{HorizontalInvertedPendulum, StateVector};
use crate::draw::{draw_line, draw_circle};
use crate::collision::{ScreenPoint, ScreenLine};
fn draw_rectangle(frame: &mut [u8], width: usize, height: usize,x: i32, y: i32, rect_width: i32, rect_height: i32) {
    let top_left = ScreenPoint { x: x as i32, y: y as i32 };
    let top_right = ScreenPoint { x: (x + rect_width) as i32, y: y as i32 };
    let bottom_left = ScreenPoint { x: x as i32, y: (y + rect_height) as i32 };
    let bottom_right = ScreenPoint { x: (x + rect_width) as i32, y: (y + rect_height) as i32 };

    let top_line = ScreenLine { point1: top_left, point2: top_right };
    let bottom_line = ScreenLine { point1: bottom_left, point2: bottom_right };
    let left_line = ScreenLine { point1: top_left, point2: bottom_left };
    let right_line = ScreenLine { point1: top_right, point2: bottom_right };

    draw_line(frame, width, height, &top_line);
    draw_line(frame, width, height, &bottom_line);
    draw_line(frame, width, height, &left_line);
    draw_line(frame, width, height, &right_line);
}

pub fn draw_pendulum(frame: &mut [u8], width: usize, height: usize, state: &StateVector, shape: &HorizontalInvertedPendulum) {
    let pendulum_length = shape.pendulum_length * 300.0;
    let cart_width = pendulum_length / 3.0;
    let cart_height = cart_width / 2.0;
    let cart_x = (width as f32) / 1.5  + state.cart_position * 300.0;
    let cart_y = 360f32;
    draw_rectangle(frame, width, height,
        (cart_x - cart_width / 2.0) as i32,
        (cart_y - cart_height / 2.0) as i32,
        cart_width as i32,
        cart_height as i32);
    let pendulum_x = cart_x + pendulum_length * -state.pendulum_angle.sin();
    let pendulum_y = cart_y + pendulum_length * state.pendulum_angle.cos();
    let pendulum_line = ScreenLine {
        point1: ScreenPoint {
            x: cart_x as i32,
            y: height as i32 - cart_y as i32,
        },
        point2: ScreenPoint {
            x: pendulum_x as i32,
            y: height as i32 - pendulum_y as i32,
        },
    };
    draw_line(frame, width, height, &pendulum_line);
    draw_circle(frame, width, height,
        &ScreenPoint {x: pendulum_x as i32, y: height as i32 - pendulum_y as i32,},
        15,
        [255, 0, 0, 255]);

}