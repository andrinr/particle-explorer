use nannou::glam::Vec2;
use nannou::prelude::*;

const GRAVITY : f32 = 1000.0;

#[derive(Copy, Clone)]
pub struct Particle {
    pub position : Vec2,
    pub velocity : Vec2,
    pub acceleration : Vec2,
    pub radius : f32,
}

impl Particle {
    fn acc(self) -> Vec2 {
        // Random noise
        let mut acc : Vec2 = Vec2::new(random_f32() - 0.5, random_f32() - 0.5) * 30.0;
        // Gravity
        let d : f32 = self.position.length() + 0.5;
        acc -= self.position.clone().normalize() / d * GRAVITY;   
        return acc;
    }

    pub fn kick_drift_kick(&mut self, dt : f32) {
        // Leap-Frog Integration
        // Kick
        let v_half = self.velocity + (self.acc() + self.acceleration) * dt * 0.5;
        // Drift
        self.position += v_half * dt;
        // Kick
        self.velocity = v_half + (self.acc() + self.acceleration) * dt * 0.5;
    }

    pub fn enforce_boundary_conditions(&mut self, width : f32, height : f32) {
        // Periodic Boundary Conditions
        self.position.x -= (self.position.x > width * 0.5) as i32 as f32 * width;
        self.position.x += (self.position.x < -width * 0.5) as i32 as f32 * width;
        self.position.y -= (self.position.y > height * 0.5) as i32 as f32 * height;
        self.position.y += (self.position.y < -height * 0.5) as i32 as f32 * height;
    }
}