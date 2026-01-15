mod collision;
mod matrix;
mod draw;

use collision::{Line, Vec2d, check_collision, change_velocity_direction, change_velocity_magnitude};
use matrix::Mat;
use draw::{draw_line,draw_circle};

const SCREEN_WIDTH: u32 = 40;
const SCREEN_HEIGHT: u32 = 40;

fn main(){
    println!("start main function");
    let mut velocity = Vec2d{x:0.0,y:0.0};
    //ball
    let mut center = Vec2d{x:-0.5,y:0.5};
    let radius: f32 = 0.1;

    //descent
    let line = Line::new(Vec2d{x:-1.0,y:-1.0},Vec2d{x:1.0,y: 1.0});
    let norm_x = 0.5f32;
    let norm = Vec2d{x:-norm_x.sqrt(),y:norm_x.sqrt()};

    //x wall
    let line_horizontal = Line::new(Vec2d{x:-1.0,y:1.0},Vec2d{x:1.0,y: 1.0});
    let norm_horizontal = Vec2d{x:0.0,y:-1.0};


    //y wall
    let line_vertical = Line::new(Vec2d{x:-1.0,y:-1.0},Vec2d{x:-1.0,y: 1.0});
    let norm_vertical = Vec2d{x:1.0,y:0.0};

    let gravity = Vec2d{x:0.0,y:-9.8/100.0};

    loop{
        let mut screen = Mat::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);
        std::thread::sleep(std::time::Duration::from_millis(500));
        center = Vec2d{x: center.x + velocity.x, y: center.y + velocity.y};
        if check_collision(&line, &norm, &center, radius){
            velocity = change_velocity_direction(&line,&norm,velocity);
        }
        if check_collision(&line_horizontal, &norm_horizontal, &center, radius){
            velocity = change_velocity_direction(&line,&norm,velocity);
        }
        if check_collision(&line_vertical, &norm_vertical, &center, radius){
            velocity = change_velocity_direction(&line,&norm,velocity);
        }
        velocity = change_velocity_magnitude(velocity,gravity );



        draw_line(&mut screen, &line);
        draw_line(&mut screen, &line_horizontal);
        draw_line(&mut screen, &line_vertical);

        draw_circle(&mut screen,&center,2, -1);

        screen.print();
        print!("\x1B[2J\x1B[1;1H");
    }
    
}
