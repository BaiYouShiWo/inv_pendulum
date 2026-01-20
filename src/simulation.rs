use crate::physical_engine::PendulumPhysicsEngine;
use crate::inverted_pendulum::StateVector;
pub struct RealTimeSimulation {
    pub physics: PendulumPhysicsEngine,
    pub accumulator: f32,
    pub fixed_dt: f32,
    pub last_time: f32,
}

impl RealTimeSimulation {
    pub fn new() -> Self {
        Self {
            physics: PendulumPhysicsEngine::new(),
            accumulator: 0.0,
            fixed_dt: 0.01,
            last_time: 0.0,
        }
    }
    
    pub fn update(&mut self, current_time: f32, control_force: f32) {
        // let frame_time = current_time - self.last_time;
        // self.last_time = current_time;
        
        // self.accumulator += frame_time;
        self.physics.step(control_force);
        //while self.accumulator >= self.fixed_dt {
        //     self.accumulator -= self.fixed_dt;
        // }
            

    }
    
    pub fn get_render_state(&self) -> StateVector {
        self.physics.get_state().clone()
    }
}