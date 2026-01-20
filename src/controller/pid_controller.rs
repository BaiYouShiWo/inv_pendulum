pub struct PIDController {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub integral: f32,
    pub previous_error: f32,
    pub integral_limit: f32,
    pub output_limit: f32,
    pub dt: f32,
}

impl PIDController {
    pub fn new(kp: f32, ki: f32, kd: f32, dt: f32) -> Self {
        Self {
            kp, ki, kd,
            integral: 0.0,
            previous_error: 0.0,
            integral_limit: 100.0,
            output_limit: 50.0,
            dt,
        }
    }
    
    pub fn update(&mut self, error: f32) -> f32 {
        // P
        let proportional = self.kp * error;
        
        // I
        self.integral += error * self.dt;
        self.integral = self.integral.clamp(-self.integral_limit, self.integral_limit);
        let integral = self.ki * self.integral;
        
        // D
        let derivative = self.kd * (error - self.previous_error) / self.dt;
        self.previous_error = error;
        
        // output
        let output = proportional + integral + derivative;
        output.clamp(-self.output_limit, self.output_limit)
    }
    
    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.previous_error = 0.0;
    }
}
