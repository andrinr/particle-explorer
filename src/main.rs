extern crate nannou;

use nannou::prelude::*;
use nannou::glam::Vec2;

const GRAVITY : f32 = 1000.0;
const MAX_DEPTH : i32 = 16;
const PARTICLE_COUNT : usize = 1<<12;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    particles : [Particle; 1<<12],
}

#[derive(Copy, Clone)]
struct Particle {
    position : Vec2,
    velocity : Vec2
}

impl Particle {
    fn acc(self, dt : f32) -> Vec2 {
        // Random noise
        let mut acc : Vec2 = Vec2::new(random_f32() - 0.5, random_f32() - 0.5) * 30.0 * dt;
        // Gravity
        let d : f32 = self.position.length() + 0.5;
        acc -= self.position.clone().normalize() / d * GRAVITY * dt;   
    
        return acc;
    }

    fn kick_drift_kick(mut self, dt : f32) {
        // Leap-Frog Integration
        // Kick
        let v_half = self.velocity + self.acc(dt) * dt * 0.5;
        // Drift
        self.position += v_half * dt;
        // Kick
        self.velocity = v_half + self.acc(dt) * dt * 0.5;
    }

    fn enforce_boundary_conditions(mut self, width : f32, height : f32) {
        // Periodic Boundary Conditions
        self.position.x -= (self.position.x > width * 0.5) as i32 as f32 * width;
        self.position.x += (self.position.x < -width * 0.5) as i32 as f32 * width;
        self.position.y -= (self.position.y > height * 0.5) as i32 as f32 * height;
        self.position.y += (self.position.y < -height * 0.5) as i32 as f32 * height;
    }
}

struct Cell {
    center : Vec2,
    size : Vec2,
    depth : i32,
    start : i32, 
    end : i32,
    child_a : Option<Box<Cell>>,
    child_b : Option<Box<Cell>>,
}


impl Cell {
    // Split domain
    fn split(self, mut particles : [Particle; PARTICLE_COUNT]) -> [Particle; PARTICLE_COUNT] {
        let next_depth = self.depth + 1;

        let dim = (self.size[0] > self.size[1]) as usize;

        let split = self.center[dim];

        let n_left = 0;

        while(n_left - (self.end - self.start) > 1) {
            n_left += 
        }

        return particles;
    }
}

trait Inner {

}

fn model(app: &App) -> Model {

    let window = app.main_window();
    let win = window.rect();
    let h = win.h();
    let w = win.w();

    let p = Vec2::new(10.0, 0.0);
    let v = Vec2::new(0.0, 0.0);
    let mut particles : [Particle; PARTICLE_COUNT] = [Particle{position : p, velocity : v}; PARTICLE_COUNT];

    for (_i, particle) in particles.iter_mut().enumerate() {
        particle.velocity.x = 100.0 * (random_f32() - 0.5) * (random_f32() - 0.5);
        particle.velocity.y = 100.0 * (random_f32() - 0.5) * (random_f32() - 0.5);

        particle.position.x = w * (random_f32() - 0.5);
        particle.position.y = h * (random_f32() - 0.5);
    }

    println!("length: {}", particles.len());

    Model {particles : particles}
}

fn update(app: &App, _model: &mut Model, _update: Update) {

    let dt : f32 = (_update.since_last.subsec_millis() as f32) * 0.001;

    let window = app.main_window();
    let win = window.rect();
    let h = win.h();
    let w = win.w();

    for (_i, particle) in _model.particles.iter_mut().enumerate() {
        particle.kick_drift_kick(dt);
        particle.enforce_boundary_conditions(w, h);
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();

    // set background to blue
    draw.background().color(BLACK);

    for particle in _model.particles {
        draw.ellipse().color(WHITE).x_y(particle.position.x, particle.position.y).radius(1.0);
    }

    // put everything on the frame
    draw.to_frame(_app, &frame).unwrap();

    //frame.clear(PURPLE)
}
