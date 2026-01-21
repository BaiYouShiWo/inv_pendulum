use crate::inverted_pendulum::{HorizontalInvertedPendulum, StateVector};
use std::f32::consts::PI;
pub struct PendulumPhysicsEngine {
    pub pendulum: HorizontalInvertedPendulum,
    pub time: f32,
    pub max_time: f32,
}

impl PendulumPhysicsEngine {
    pub fn new() -> Self {
        let pendulum = HorizontalInvertedPendulum {
            state: StateVector {
                cart_position: 0.0,
                cart_velocity: 0.0,
                pendulum_angle: 2.8,  // initial angle
                pendulum_ang_vel: 0.0,
            },
            cart_mass: 0.5,
            ball_mass: 1.0,
            pendulum_length: 1.0,
            gravity: 9.81,
            dt: 0.01,  // 10ms
        };
        
        Self {
            pendulum,
            time: 0.0,
            max_time: -1.0,
        }
    }
    
    pub fn step(&mut self, control_force: f32) {
        if self.time > self.max_time {
            self.pendulum.rk4_step(control_force);
            self.time += self.pendulum.dt;
        }
    }
    
    pub fn get_state(&self) -> &StateVector {
        &self.pendulum.state
    }
    
    pub fn is_failed(&self) -> bool {
        self.pendulum.state.pendulum_angle.abs() > PI / 2.0
    }
}