use std::f32::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct StateVector {
    pub cart_position: f32,
    pub cart_velocity: f32,
    pub pendulum_angle: f32,
    pub pendulum_ang_vel: f32,
}

pub struct HorizontalInvertedPendulum {
    pub state: StateVector,
    pub cart_mass: f32,
    pub ball_mass: f32,
    pub pendulum_length: f32,
    pub gravity: f32,
    pub dt: f32,
}

impl HorizontalInvertedPendulum {
    pub fn compute_derivatives(&self, state: &StateVector, control_force: f32) -> StateVector {
        let m_c = self.cart_mass;
        let m_b = self.ball_mass;
        let l = self.pendulum_length;
        let g = self.gravity;   
        
        let x = state.cart_position;
        let x_dot = state.cart_velocity;
        let theta = state.pendulum_angle;
        let theta_dot = state.pendulum_ang_vel;
        

        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        
        let total_mass = m_c + m_b;
        let sin_cos = sin_theta * cos_theta;

        let denominator = total_mass - m_b * cos_theta * cos_theta;

        let cart_accel= (control_force + m_b * g * sin_cos - m_b * l * theta_dot * theta_dot * sin_theta) / denominator;
        
        let pendulum_accel = (total_mass * g * sin_theta + control_force * cos_theta - m_b * l * theta_dot * theta_dot * sin_cos) / (l * denominator);

        
        StateVector {
            cart_position: x_dot,                    // dx/dt = v
            cart_velocity: cart_accel,               // dv/dt = a
            pendulum_angle: theta_dot,               // dθ/dt = ω
            pendulum_ang_vel: pendulum_accel,        // dω/dt = α
        }
    }
}

impl HorizontalInvertedPendulum {
    /// RK4
    pub fn rk4_step(&mut self, control_force: f32) {
        let dt = self.dt;
        let current_state = &self.state;
        
        // k1
        let k1 = self.compute_derivatives(&current_state, control_force);
        
        // k2
        let state2 = StateVector {
            cart_position: current_state.cart_position + k1.cart_position * dt * 0.5,
            cart_velocity: current_state.cart_velocity + k1.cart_velocity * dt * 0.5,
            pendulum_angle: current_state.pendulum_angle + k1.pendulum_angle * dt * 0.5,
            pendulum_ang_vel: current_state.pendulum_ang_vel + k1.pendulum_ang_vel * dt * 0.5,
        };
        let k2 = self.compute_derivatives(&state2, control_force);
        
        // k3
        let state3 = StateVector {
            cart_position: current_state.cart_position + k2.cart_position * dt * 0.5,
            cart_velocity: current_state.cart_velocity + k2.cart_velocity * dt * 0.5,
            pendulum_angle: current_state.pendulum_angle + k2.pendulum_angle * dt * 0.5,
            pendulum_ang_vel: current_state.pendulum_ang_vel + k2.pendulum_ang_vel * dt * 0.5,
        };
        let k3 = self.compute_derivatives(&state3, control_force);
        
        // k4
        let state4 = StateVector {
            cart_position: current_state.cart_position + k3.cart_position * dt,
            cart_velocity: current_state.cart_velocity + k3.cart_velocity * dt,
            pendulum_angle: current_state.pendulum_angle + k3.pendulum_angle * dt,
            pendulum_ang_vel: current_state.pendulum_ang_vel + k3.pendulum_ang_vel * dt,
        };
        let k4 = self.compute_derivatives(&state4, control_force);
        
        // final
        self.state.cart_position += (k1.cart_position + 2.0*k2.cart_position + 2.0*k3.cart_position + k4.cart_position) * dt / 6.0;
        self.state.cart_velocity += (k1.cart_velocity + 2.0*k2.cart_velocity + 2.0*k3.cart_velocity + k4.cart_velocity) * dt / 6.0;
        self.state.pendulum_angle += (k1.pendulum_angle + 2.0*k2.pendulum_angle + 2.0*k3.pendulum_angle + k4.pendulum_angle) * dt / 6.0;
        self.state.pendulum_ang_vel += (k1.pendulum_ang_vel + 2.0*k2.pendulum_ang_vel + 2.0*k3.pendulum_ang_vel + k4.pendulum_ang_vel) * dt / 6.0;
        
        //normalize angle
        self.state.pendulum_angle = self.normalize_angle(self.state.pendulum_angle);
    }
    
    fn normalize_angle(&self, angle: f32) -> f32 {
        let mut normalized = angle;
        while normalized > PI {
            normalized -= 2.0 * PI;
        }
        while normalized < -PI {
            normalized += 2.0 * PI;
        }
        normalized
    }
}