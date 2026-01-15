use crate::collision::{Line, Vec2d};
use crate::matrix::Mat;
use crate::draw::ndcToScreen::ndc_line_to_screen;

pub fn draw_line(canvas: &mut Mat, line: &Line) {
    let (width, height) = canvas.shape();
    let map_target: (i32, i32) = (width as i32, height as i32);
    
    let screen_line = match ndc_line_to_screen(line, map_target) {
        Some(line) => line,
        None => {
            eprintln!("Warning: Failed to convert NDC line to screen coordinates");
            return;
        }
    };
    
    let (x0, y0) = (screen_line.0.0, screen_line.0.1);
    let (x1, y1) = (screen_line.1.0, screen_line.1.1);
    
    //println!("Drawing line from ({}, {}) to ({}, {})", x0, y0, x1, y1);
    
    let (start_x, start_y, end_x, end_y) = if x0 > x1 || (x0 == x1 && y0 > y1) {
        (x1, y1, x0, y0)
    } else {
        (x0, y0, x1, y1)
    };
    
    let dx = end_x - start_x;
    let dy = end_y - start_y;
    
    if !is_valid_coord(start_x, start_y, width, height) || 
       !is_valid_coord(end_x, end_y, width, height) {
        eprintln!("Warning: Line coordinates out of canvas bounds");
        return;
    }
    
    if dx == 0 {
        draw_vertical_line(canvas, start_x, start_y, end_y);
        return;
    }
    
    if dy == 0 {
        draw_horizontal_line(canvas, start_x, end_x, start_y);
        return;
    }
    
    let abs_dx = dx.abs();
    let abs_dy = dy.abs();
    
    if abs_dx >= abs_dy {
        draw_line_shallow(canvas, start_x, start_y, end_x, end_y, dx, dy, abs_dx, abs_dy);
    } else {
        draw_line_steep(canvas, start_x, start_y, end_x, end_y, dx, dy, abs_dx, abs_dy);
    }
}

fn is_valid_coord(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height
}

fn draw_line_shallow(
    canvas: &mut Mat, 
    start_x: i32, start_y: i32, 
    end_x: i32, end_y: i32,
    dx: i32, dy: i32, 
    abs_dx: i32, abs_dy: i32
) {
    let width = canvas.shape().0;
    let height = canvas.shape().1;
    
    let mut x = start_x;
    let mut y = start_y as f32;
    let slope = dy as f32 / dx as f32;
    
    while x <= end_x {
        // 边界检查
        if is_valid_coord(x, y.round() as i32, width, height) {
            let y_int = y.round() as i32;
            canvas.set(x, y_int, '@');
        }
        
        x += 1;
        y += slope;
    }
}

fn draw_line_steep(
    canvas: &mut Mat, 
    start_x: i32, start_y: i32, 
    end_x: i32, end_y: i32,
    dx: i32, dy: i32, 
    abs_dx: i32, abs_dy: i32
) {
    let width = canvas.shape().0;
    let height = canvas.shape().1;
    
    let mut y = start_y;
    let mut x = start_x as f32;
    let inv_slope = dx as f32 / dy as f32;
    
    while y <= end_y {
        // 边界检查
        if is_valid_coord(x.round() as i32, y, width, height) {
            let x_int = x.round() as i32;
            canvas.set(x_int, y, '@');
        }
        
        y += 1;
        x += inv_slope;
    }
}

fn draw_vertical_line(canvas: &mut Mat, x: i32, y0: i32, y1: i32) {
    let (start_y, end_y) = if y0 < y1 { (y0, y1) } else { (y1, y0) };
    
    for y in start_y..=end_y {
        if is_valid_coord(x, y, canvas.shape().0, canvas.shape().1) {
            canvas.set(x, y, '@');
        }
    }
}

fn draw_horizontal_line(canvas: &mut Mat, x0: i32, x1: i32, y: i32) {
    let (start_x, end_x) = if x0 < x1 { (x0, x1) }  else { (x1, x0) };
    
    for x in start_x..=end_x {
        if is_valid_coord(x, y, canvas.shape().0, canvas.shape().1) {
            canvas.set(x, y, '@');
        }
    }
}