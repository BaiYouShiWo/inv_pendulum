mod collision;
mod matrix;
mod draw;

use collision::{Vec2d, CollisionLine, Boundaries};
use draw::{draw_line,draw_circle};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::time::{Instant};

use crate::collision::{Ball, Balls, ScreenLine, ScreenPoint}; 


const SCREEN_WIDTH: usize = 720;
const SCREEN_HEIGHT: usize = 640;

fn main(){
    println!("start main function");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pixel Drawing Example")
        .with_inner_size(winit::dpi::LogicalSize::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32))
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, &window);
    let mut canvas = Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).unwrap();
    
    //ball
    let mut balls = Balls{
        balls: vec![
            Ball{
                position: ScreenPoint {x:50, y:30},
                radius: 15.0,
                velocity: Vec2d{x:1.0,y:-0.87},
            },
            // Ball{
            //     position: ScreenPoint {x:80, y:80},
            //     radius: 15.0,
            //     velocity: Vec2d{x:0.0,y:0.0},
            // },
            // Ball{
            //     position: ScreenPoint {x:140, y:140},
            //     radius: 15.0,
            //     velocity: Vec2d{x:0.0,y:0.0},
            // },
        ]
    };

    let left_top_point = ScreenPoint{x:0, y:0};
    let right_top_point = ScreenPoint{x:400, y:0};
    let left_bottom_point = ScreenPoint{x:0, y:300};

    let left_wall = ScreenLine{
    point1: left_top_point,
    point2: left_bottom_point};
    let top_wall = ScreenLine{
    point1: left_top_point,
    point2: right_top_point};
    let slope_wall = ScreenLine{
    point1: left_bottom_point,
    point2: right_top_point};

    let boundaries = Boundaries {
        lines: vec![
            CollisionLine {
                line: slope_wall,
                normal: Vec2d{x:-3.0,y:-4.0}.normalize(),
            },
            CollisionLine {
                line: top_wall,
                normal: Vec2d{x:0.0,y:1.0},
            },
            CollisionLine {
                line: left_wall,
                normal: Vec2d{x:1.0,y:0.0},
            },
        ],
        damping_factor: 0.99,
    };

    let gravity = Vec2d{x:0.0,y:9.8};

    let mut last_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(_) => {
                let current_time = Instant::now();
                let dt = current_time.duration_since(last_time).as_secs_f32();
                last_time = current_time;
                
                // update physical state
                for ball in &mut balls.balls{
                    ball.velocity = collision::change_velocity_magnitude(ball.velocity,gravity, dt);
                    ball.update_position(dt);
                }

                for coll_line in &boundaries.lines{
                    for ball in &mut balls.balls{
                        if collision::check_collision(&coll_line.line, &coll_line.normal, &ball.position, ball.radius){
                            ball.velocity = collision::change_velocity_direction(
                                &coll_line.normal,
                                ball.velocity) * boundaries.damping_factor;
                        let temp = coll_line.normal * (ball.radius / 2.0);
                        ball.position = ball.position + ScreenPoint{x:temp.x as i32, y:temp.y as i32};
                        }
                    }
                }
                // for i in 0..balls.balls.len() {
                //     for j in (i + 1)..balls.balls.len() {
                //         let (left, right) = balls.balls.split_at_mut(j);
                //         let ball = &mut left[i];
                //         let other_ball = &mut right[0];
                        
                //         if collision::check_ball_ball_collision(&ball.position, &other_ball.position, ball.radius, other_ball.radius) {
                //             ball.velocity = collision::change_velocity_direction(
                //                 &(other_ball.position - ball.position).normalize(),
                //                 ball.velocity
                //             ) * boundaries.damping_factor;
                            
                //             other_ball.velocity = collision::change_velocity_direction(
                //                 &(ball.position - other_ball.position).normalize(),
                //                 other_ball.velocity
                //             ) * boundaries.damping_factor;
                //         }
                //     }
                // }
                

                
                let frame = canvas.frame_mut();
                frame.fill(0);
                
                let mut lines_to_draw = Vec::new();
                let mut circles_to_draw = Vec::new();


                for coll_line in &boundaries.lines{
                    lines_to_draw.push(coll_line.line);
                }
                for ball in &balls.balls{
                    let center = ball.position;
                    let radius = ball.radius;
                    circles_to_draw.push((center, radius, -1));
                }

                // draw lines and balls
                for line in lines_to_draw {
                    draw_line(frame, SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize, &line);
                }
                for (center, radius, color) in circles_to_draw {
                    draw_circle(frame, SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize, &center, radius as i32, color);
                }
                
                if canvas.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
