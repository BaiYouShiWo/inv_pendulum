use crate::collision::{ScreenLine};

// pub fn draw_line(frame: &mut [u8], width: usize, height: usize, line: &ScreenLine) {
//     // 直接在这里进行Y轴翻转
//     let (x0, y0) = (line.point1.x as i32, line.point1.y as i32);
//     let (x1, y1) = (line.point2.x as i32, line.point2.y as i32);
    
//     // Y轴翻转：y = height - 1 - y
//     let flipped_y0 = (height as i32 - 1) - y0;
//     let flipped_y1 = (height as i32 - 1) - y1;
    
//     bresenham_line(frame, width, height, x0, flipped_y0, x1, flipped_y1);
// }

pub fn draw_line(frame: &mut [u8], width: usize, height: usize, line: &ScreenLine) {
    let (x0, y0) = (line.point1.x, line.point1.y);
    let (x1, y1) = (line.point2.x, line.point2.y);
    
    // 使用整数Bresenham算法，避免浮点数精度问题
    bresenham_line(frame, width, height, x0, y0, x1, y1);
}

fn bresenham_line(frame: &mut [u8], width: usize, height: usize, x0: i32, y0: i32, x1: i32, y1: i32) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        set_pixel(frame, width, height, x, y);
        
        if x == x1 && y == y1 { break; }
        
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

fn set_pixel(frame: &mut [u8], width: usize, height: usize, x: i32, y: i32) {
    if x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height {
        let idx = (y as usize * width + x as usize) * 4;
        if idx + 3 < frame.len() {
            frame[idx..idx+4].copy_from_slice(&[255, 255, 255, 255]);
        }
    }
}