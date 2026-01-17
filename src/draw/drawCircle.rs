use crate::{collision::ScreenPoint};
use std::f32::consts::PI;

// pub fn draw_circle(frame: &mut [u8], width: usize, height: usize, point: &ScreenPoint, radius:i32, thickness: i32){
//     let center_x = point.x;
//     let center_y = height as i32 - point.y - 1;

//     for y in -radius..=radius {
//         let current_y = center_y + y;
        
//         if current_y < 0 || current_y >= height as i32 {
//             continue;
//         }
        
//         let y_squared = (y * y) as f32;
//         let radius_squared = (radius * radius) as f32;
        
//         if y_squared > radius_squared {
//             continue;
//         }
        
//         let max_x = (radius_squared - y_squared).sqrt() as i32;
        
//         let start_x = center_x - max_x;
//         let end_x = center_x + max_x;
        
//         for x in start_x..=end_x {
//             if x >= 0 && x < width as i32 {
//                 let idx = (current_y as usize * width as usize + x as usize) * 4;
//                 frame[idx..idx+4].copy_from_slice(&[255, 255, 255, 255]);
//             }
//         }
//     }
// }

pub fn draw_circle(frame: &mut [u8], width: usize, height: usize, point: &ScreenPoint, radius:i32, thickness: i32){
    let center_x = point.x;
    let center_y = point.y;

    for y in -radius..=radius {
        let current_y = center_y + y;
        
        if current_y < 0 || current_y >= height as i32 {
            continue;
        }
        
        let y_squared = (y * y) as f32;
        let radius_squared = (radius * radius) as f32;
        
        if y_squared > radius_squared {
            continue;
        }
        
        let max_x = (radius_squared - y_squared).sqrt() as i32;
        
        let start_x = center_x - max_x;
        let end_x = center_x + max_x;
        
        for x in start_x..=end_x {
            if x >= 0 && x < width as i32 {
                let idx = (current_y as usize * width as usize + x as usize) * 4;
                frame[idx..idx+4].copy_from_slice(&[255, 255, 255, 255]);
            }
        }
    }
}