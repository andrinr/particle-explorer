extern crate nannou;

mod tree;

use nannou::prelude::*;
use nannou::glam::Vec2;

const PARTICLE_COUNT : usize = 1<<6;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    particles : [tree::particle::Particle; PARTICLE_COUNT]
}

fn model(app: &App) -> Model {

    let window = app.main_window();
    let win = window.rect();
    let h = win.h();
    let w = win.w();

    let p = Vec2::new(10.0, 0.0);
    let v = Vec2::new(0.0, 0.0);

    let mut particles : [tree::particle::Particle; PARTICLE_COUNT] = [tree::particle::Particle{position : p, velocity : v}; PARTICLE_COUNT];

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
