use crate::collision::{Line, Vec2d};
pub fn ndc_line_to_screen(line: &Line, screen_size: (i32, i32)) -> Option<((i32, i32), (i32, i32))> {
    let start = ndc_to_screen(&line.point1, screen_size)?;
    let end = ndc_to_screen(&line.point2, screen_size)?;
    
    let (width, height) = screen_size;
    let start = (
        start.0.max(0).min(width - 1),
        start.1.max(0).min(height - 1)
    );
    let end = (
        end.0.max(0).min(width - 1),
        end.1.max(0).min(height - 1)
    );
    
    Some((start, end))
}

pub fn ndc_to_screen(point: &Vec2d, screen_size: (i32, i32)) -> Option<(i32, i32)> {
    if !(-1.0..=1.0).contains(&point.x) || !(-1.0..=1.0).contains(&point.y) {
        return None;
    }
    
    let x = ((point.x + 1.0) / 2.0 * screen_size.0 as f32).round() as i32;
    let y = ((point.y + 1.0) / 2.0 * screen_size.1 as f32).round() as i32;
    
    Some((x, y))
}