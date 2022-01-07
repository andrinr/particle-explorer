extern crate nannou;

use nannou::prelude::*;
use nannou::glam::Vec2;

const GRAVITY : f32 = 1000.0;
const MAX_DEPTH : i32 = 16;
const PARTICLE_COUNT : usize = 1<<4;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Test {
    a : i32,
}

impl Test {
    fn incr(mut self, b : i32) {
        self.a += b;
    }
}

struct Model {
    particles : [Particle; PARTICLE_COUNT],
}

#[derive(Copy, Clone)]
struct Particle {
    position : Vec2,
    velocity : Vec2
}

impl Particle {
    fn acc(self) -> Vec2 {
        // Random noise
        let mut acc : Vec2 = Vec2::new(random_f32() - 0.5, random_f32() - 0.5) * 3000.0;
        // Gravity
        //let d : f32 = self.position.length() + 0.5;
        //acc -= self.position.clone().normalize() / d * GRAVITY * dt;   
        return acc;
    }

    fn kick_drift_kick(&mut self, dt : f32) {
        // Leap-Frog Integration
        // Kick
        println!("pos : {}", self.position);
        let v_half = self.velocity + self.acc() * dt * 0.5;
        // Drift
        println!("v half : {}", v_half);
        self.position += v_half * dt;
        println!("pos : {}", self.position);
        //println!("v half : {}", self.v_half);
        // Kick
        self.velocity = v_half + self.acc() * dt * 0.5;
    }

    fn enforce_boundary_conditions(&mut self, width : f32, height : f32) {
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
    start_index : i32, 
    end_index : i32,
    child_a : Option<Box<Cell>>,
    child_b : Option<Box<Cell>>,
}

impl Cell {
    // Split domain
    fn split(mut self, mut particles : [Particle; PARTICLE_COUNT]) -> [Particle; PARTICLE_COUNT] {
        let next_depth = self.depth + 1;

        if next_depth == MAX_DEPTH {
            return particles;
        }

        let dimension = (self.size[0] > self.size[1]) as usize;
        let half_count : i32  = (self.end_index - self.start_index) / 2;
        let mut step : f32 = self.size[dimension] / 2.0;
        let mut split : f32 = self.center[dimension];
        
        let mut left_count : i32 = 0;

        // binary search for propper split
        let particles_slice : &[Particle] = &particles[self.start_index as usize..self.end_index as usize];
        loop {
            for (_i, particle) in particles_slice.iter().enumerate() {
                left_count += (particle.position[dimension] < split) as i32 + self.start_index;
            }

            step /= 2.0;

            split += if left_count < half_count { step } else { -step};

            if abs(left_count - half_count) <= 1 { break; }
        }
        
        // TODO: reshuffle array

        // Define new child cells
        let mut center_a : Vec2 = Vec2::new(0.0, 0.0);
        let mut center_b : Vec2 = Vec2::new(0.0, 0.0);

        center_a[1 - dimension] = self.center[1 - dimension];
        center_b[1 - dimension] = self.center[1- dimension];

        center_a[dimension] = self.center[dimension] - self.size[dimension] / 2.0 + split / 2.0;
        center_b[dimension] = self.center[dimension] + self.size[dimension] / 2.0 - split / 2.0;

        let size_a : Vec2 = self.center.clone() - center_a * 2.0;
        let size_b : Vec2 = center_b.clone() - self.center * 2.0;

        self.child_a = Some(Box::new(Cell {
            center : center_a,
            size : size_a,
            depth : next_depth,
            start_index : self.start_index,
            end_index : self.start_index + left_count,
            child_a : None,
            child_b : None
        }));

        self.child_b = Some(Box::new(Cell {
            center : center_b,
            size : size_b,
            depth : next_depth,
            start_index : self.start_index + left_count,
            end_index : self.end_index,
            child_a : None,
            child_b : None
        }));

        return particles;
    }
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

fn update(app: &App, model: &mut Model, update: Update) {

    let dt : f32 = (update.since_last.subsec_millis() as f32) * 0.001;

    let window = app.main_window();
    let win = window.rect();
    let h = win.h();
    let w = win.w();
    //println!("Particle 0-0 {}", model.particles[0].position);
    model.particles[0].kick_drift_kick(dt);
    //println!("Particle 0-1 {}", model.particles[0].positiosn);
    
    for particle in model.particles.iter_mut() {
        particle.kick_drift_kick(dt);
        particle.enforce_boundary_conditions(w, h);
    }
    //println!("Update {}", dt);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLACK);

    for particle in model.particles {
        draw.ellipse().color(WHITE).x_y(particle.position.x, particle.position.y).radius(1.0);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    //frame.clear(PURPLE)
}
