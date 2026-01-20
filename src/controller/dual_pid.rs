use rand::rand_core::le;

use crate::inverted_pendulum::StateVector;
use crate::controller::pid_controller::PIDController;

pub struct DualLoopPID {
    // angle
    angle_pid: PIDController,
    // position
    position_pid: PIDController,
    
    // controller parameters
    max_force: f32,
}

impl DualLoopPID {
    pub fn new(dt: f32) -> Self {
        Self {
            angle_pid: PIDController::new(100.0, 20.0, 10.0, dt),
            position_pid: PIDController::new(5.0, 1.0, 2.0, dt),
            max_force: 20.0,
        }
    }
    
    pub fn compute_control(&mut self, state: &StateVector, target_position: f32) -> f32 {
        let angle_error = -state.pendulum_angle;
        
        let angle_force = self.angle_pid.update(angle_error);
        
        let position_error = target_position - state.cart_position;
        let position_force = self.position_pid.update(position_error);
        
        let mut total_force = angle_force + position_force * 0.4;
        
        total_force = total_force.clamp(-self.max_force, self.max_force);
        
        if cfg!(debug_assertions) {
            println!("Angle err: {:.3}, Pos err: {:.3}, Force: {:.3}", 
                     angle_error, position_error, total_force);
        }
        
        total_force
    }
    
    pub fn reset(&mut self) {
        self.angle_pid.reset();
        self.position_pid.reset();
    }
}