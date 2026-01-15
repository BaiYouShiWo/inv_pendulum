use crate::{collision::Vec2d, matrix::Mat};
use crate::draw::ndcToScreen::ndc_to_screen;
use std::f32::consts::PI;

pub fn draw_circle(canvas: &mut Mat, point: &Vec2d, radius:i32, thickness: i32){

    let (width, height) = canvas.shape();
    let (center_x, center_y) = if let Some(coords) = ndc_to_screen(point, (width as i32, height as i32)) {
        coords
    } else {
        eprintln!("Warning: Failed to convert NDC point to screen coordinates");
        return; // 或者提供默认值
    };
    // 遍历所有可能的 y 坐标
    for y in -radius..=radius {
        let current_y = center_y + y;
        
        // 检查 y 坐标是否在画布范围内
        if current_y < 0 || current_y >= height as i32 {
            continue;
        }
        
        // 计算当前 y 对应的圆的边界 x 坐标
        // 使用圆的方程: x² + y² = r²
        let y_squared = (y * y) as f32;
        let radius_squared = (radius * radius) as f32;
        
        if y_squared > radius_squared {
            continue; // 在圆外
        }
        
        // 计算 x 的边界
        let max_x = (radius_squared - y_squared).sqrt() as i32;
        
        // 在当前行从左到右填充
        let start_x = center_x - max_x;
        let end_x = center_x + max_x;
        
        for x in start_x..=end_x {
            if x >= 0 && x < width as i32 {
                canvas.set(x, current_y, '*');
            }
        }
    }
}