extern crate nannou;

mod tree;

use nannou::prelude::*;
use nannou::glam::Vec2;

const PARTICLE_COUNT : usize = 1<<10;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    particles : [tree::particle::Particle; PARTICLE_COUNT],
    root : tree::Cell
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

    let mut root = tree::Cell {
        center : Vec2::new(w / 2.0, h / 2.0),
        size : Vec2::new(w, h),
        depth : 0,
        child_a : None, 
        child_b : None
    };

    Model {particles : particles, root: root}
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

    model.root = tree::Cell {
        center : Vec2::new(0.0, 0.0),
        size : Vec2::new(w, h),
        depth : 0,
        child_a : None, 
        child_b : None
    };

    model.root.split(&mut model.particles[0..PARTICLE_COUNT], 6);
}

fn view(app: &App, model: &Model, frame: Frame) {

    let window = app.main_window();
    let win = window.rect();

    let draw = app.draw();

    // set background to blue
    draw.rect().w(win.w()).h(win.h()).x_y(0.0, 0.0).color(rgba(0.0, 0.0, 0.0, 0.3));

    recursive_cell_view(&model.root, &draw);

    for particle in model.particles {
        draw.ellipse()
        .color(RED)
        .x_y(particle.position.x, particle.position.y)
        .radius(2.0)
        .resolution(10.0);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    //frame.clear(PURPLE)
}

fn recursive_cell_view(cell : &tree::Cell, draw : &Draw) {

    draw.rect()
    .no_fill()
    .stroke(WHITE)
    .stroke_weight(1.0)
    .x_y(cell.center.x, cell.center.y)
    .w(cell.size.x)
    .h(cell.size.y);

    match & cell.child_a {
        Some(x) => recursive_cell_view(x, &draw),
        None => ()
    }
    
    match & cell.child_b {
        Some(x) => recursive_cell_view(x, &draw),
        None => ()
    } 
}
