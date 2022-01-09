extern crate nannou;

use nannou::prelude::*;
use nannou::glam::Vec2;

const GRAVITY : f32 = 1000.0;
const MAX_DEPTH : i32 = 2;
const PARTICLE_COUNT : usize = 1<<6;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    particles : [Particle; PARTICLE_COUNT]
}

#[derive(Copy, Clone)]
struct Particle {
    position : Vec2,
    velocity : Vec2
}

impl Particle {
    fn acc(self) -> Vec2 {
        // Random noise
        let mut acc : Vec2 = Vec2::new(random_f32() - 0.5, random_f32() - 0.5) * 3.0;
        // Gravity
        let d : f32 = self.position.length() + 0.5;
        acc -= self.position.clone().normalize() / d * GRAVITY;   
        return acc;
    }

    fn kick_drift_kick(&mut self, dt : f32) {
        // Leap-Frog Integration
        // Kick
        let v_half = self.velocity + self.acc() * dt * 0.5;
        // Drift
        self.position += v_half * dt;
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
    child_a : Option<Box<Cell>>,
    child_b : Option<Box<Cell>>,
}

impl Cell {
    // Split domain
    fn split(mut self, mut particles : &mut[Particle]){
        let next_depth = self.depth + 1;
        let n = particles.len();

        if next_depth == MAX_DEPTH {
            return;
        }

        let dimension = (self.size[0] > self.size[1]) as usize;
        let half_count : i32  = (n as i32) / 2;
        let mut step : f32 = self.size[dimension] / 2.0;
        let mut split : f32 = self.center[dimension];
        
        let mut left_count : i32 = 0;

        println!("{},{},{},{}", dimension, half_count, step, split);

        loop {
            left_count = 0;

            for (_i, particle) in particles.iter().enumerate() {
                left_count += (particle.position[dimension] < split ) as i32;
            }
            
            // maybe swithc to parallel version
            //particles.par_iter().filter(|&p| p.position[dimension] < split).reduce(|x, y| x + y);

            if abs(left_count - half_count) <= 1 { break; }

            step /= 2.0;

            split += if left_count < half_count { step } else { -step};
        }
        
        // TODO: reshuffle array
        let mut i = n - 1;
        let mut j = 0;

        loop {
            if i == j { break ;}

            if particles[i].position[dimension] < split {
                i += 1;
                continue;
            }
            if particles[i].position[dimension] > split {
                j -= 1;
                continue;
            }

            particles.swap(i, j);
        }

        // Define new child cells
        let mut center_a : Vec2 = Vec2::new(0.0, 0.0);
        let mut center_b : Vec2 = Vec2::new(0.0, 0.0);

        center_a[1 - dimension] = self.center[1 - dimension];
        center_b[1 - dimension] = self.center[1- dimension];

        center_a[dimension] = self.center[dimension] - self.size[dimension] / 2.0 + split / 2.0;
        center_b[dimension] = self.center[dimension] + self.size[dimension] / 2.0 - split / 2.0;

        let size_a : Vec2 = self.center.clone() - center_a * 2.0;
        let size_b : Vec2 = center_b.clone() - self.center * 2.0;

        let a= Box::new(Cell {
            center : center_a,
            size : size_a,
            depth : next_depth,
            child_a : None,
            child_b : None
        });

        (*a).split(&mut particles[0 .. left_count as usize]);

        let b = Box::new(Cell {
            center : center_b,
            size : size_b,
            depth : next_depth,
            child_a : None,
            child_b : None
        });

        (*b).split(&mut particles[left_count as usize .. n]);

        self.child_a = Some(a);
        self.child_b = Some(b);
    }

    fn draw(self, app: &App) {
        
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

    let root = Cell {
        center : Vec2::new(w / 2.0, h / 2.0),
        size : Vec2::new(w, h),
        depth : 0,
        child_a : None, 
        child_b : None
    };

    root.split(&mut model.particles[0..PARTICLE_COUNT]);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLACK);

    for particle in model.particles {
        draw.ellipse()
        .color(WHITE)
        .x_y(particle.position.x, particle.position.y)
        .radius(1.0)
        .resolution(10.0);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    //frame.clear(PURPLE)
}
