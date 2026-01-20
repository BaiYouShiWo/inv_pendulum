mod inverted_pendulum;
mod physical_engine;
mod simulation;
mod controller;
mod collision;
mod draw;
//mod demo_ball_collision;

use simulation::RealTimeSimulation;


use std::time::{Duration, Instant};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const SCREEN_WIDTH: usize = 960;
const SCREEN_HEIGHT: usize = 720;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pixel Drawing Example")
        .with_inner_size(winit::dpi::LogicalSize::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32))
        .build(&event_loop)
        .unwrap();

    let mut console_timer = Instant::now();
    let mut console_counter = 0;
    let mut fps = 0.0;

    let surface_texture = SurfaceTexture::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, &window);
    let mut canvas = Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).unwrap();

    let mut sim = RealTimeSimulation::new();
    let mut controller = controller::dual_pid::DualLoopPID::new(0.01);

    let first_time = Instant::now();
    let mut last_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(_) => {
                std::thread::sleep(Duration::from_millis(9));
                let current_time = Instant::now();
                let dt = current_time.duration_since(last_time).as_secs_f32();
                let duration = current_time.duration_since(first_time);
                last_time = current_time;

                // Update simulation
                let control_force = controller.compute_control(
                    sim.physics.get_state(),
                    0.0,
                );
                sim.update(duration.as_secs() as f32, control_force);
                // println!("duration{},{:?}",duration.as_secs(),state);

                // Draw frame
                let frame = canvas.frame_mut();
                frame.fill(0);
                let state = sim.get_render_state();
                draw::draw_pendulum(frame, SCREEN_WIDTH, SCREEN_HEIGHT, &state, &sim.physics.pendulum);
                if canvas.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                // Performance logging
                console_counter += 1;
                if console_timer.elapsed() >= std::time::Duration::from_secs(2) {
                    fps = console_counter as f32 / 2.0;
                    println!("=== Performance Stats ===");
                    println!("FPS: {:.1}", fps);
                    println!("Average Frame Time: {:.2}ms", 1000.0 / fps);
                    println!("Physics DT: {:.3}ms", dt * 1000.0);
                    println!("========================");
                    
                    console_counter = 0;
                    console_timer = Instant::now();
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
