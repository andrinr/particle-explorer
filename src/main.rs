extern crate nannou;

use nannou::prelude::*;
use nannou::glam::Vec2;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

#[derive(Copy, Clone)]
struct Particle{
    p : Vec2,
    v : Vec2
}
struct Model {
    particles : [Particle; 1<<12],
}

fn model(_app: &App) -> Model {

    let p = Vec2::new(10.0, 0.0);
    let v = Vec2::new(0.0, 0.0);
    let mut particles : [Particle; 1<<12] = [Particle{p : p, v : v}; 1<<12];

    for (_i, particle) in particles.iter_mut().enumerate() {
        particle.v.x = 100.0 * (random_f32() - 0.5) * (random_f32() - 0.5);
        particle.v.y = 100.0 * (random_f32() - 0.5) * (random_f32() - 0.5);

        particle.p.x = 400.0 * (random_f32() - 0.5) * (random_f32() - 0.5);
        particle.p.y = 400.0 * (random_f32() - 0.5) * (random_f32() - 0.5);
    }

    println!("length: {}", particles.len());

    Model {particles : particles}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {

    let dt : f32 = (_update.since_last.subsec_millis() as f32) * 0.001;

    for (_i, particle) in _model.particles.iter_mut().enumerate() {
        // Leap-Frog Integration
        // Kick
        let v_half = particle.v + acc(particle.p, dt) * dt * 0.5;
        // Drift
        particle.p += v_half * dt;
        // Kick
        particle.v = v_half + acc(particle.p, dt) * dt * 0.5;
    }
}

fn acc(pos : Vec2, dt : f32) -> Vec2 {
    // Random noise
    let mut acc : Vec2 = Vec2::new(random_f32() - 0.5, random_f32() - 0.5) * 30.0 * dt;
    // Gravity
    let d : f32 = pos.length() + 0.5;
    const G : f32 = 10000.0;
    acc -= pos.clone().normalize() / d * G * dt;   

    return acc;
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();

    // set background to blue
    //draw.background().color(BLACK);

    for particle in _model.particles {
        draw.ellipse().color(WHITE).x_y(particle.p.x, particle.p.y).radius(1.0);
    }

    // put everything on the frame
    draw.to_frame(_app, &frame).unwrap();

    //frame.clear(PURPLE)
}
