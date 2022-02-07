extern crate nannou;

mod tree;

use nannou::prelude::*;
use nannou::glam::Vec2;

const PARTICLE_MAX_RADIUS : f32 = 10.0;
const PARTICLE_MIN_RADIUS : f32 = 10.0;
const PARTICLE_COUNT : usize = 1<<11;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1200, 1200)
        .run();
}

struct Model {
    particles : Vec<tree::particle::Particle>,
    particles_indices : Vec<usize>,
    particles_tracer : Vec<usize>,
    root : tree::Cell,
    i : i32
}

fn model(app: &App) -> Model {
    let window = app.main_window();
    let win = window.rect();
    let h = win.h();
    let w = win.w();

    let mut particles : Vec<tree::particle::Particle> = Vec::with_capacity(PARTICLE_COUNT);

    let mut particles_indices : Vec<usize> = Vec::with_capacity(PARTICLE_COUNT);
    let mut particles_tracer : Vec<usize> = Vec::with_capacity(PARTICLE_COUNT);

    for i in 0..PARTICLE_COUNT {
        let r = h / 4.0;
        let velocity : Vec2 = Vec2::new(
            100.0 * (random_f32() - 0.5) * (random_f32() - 0.5),
            100.0 * (random_f32() - 0.5) * (random_f32() - 0.5)
        );

        let position : Vec2 = Vec2::new(
            (i as f32 / PARTICLE_COUNT as f32 * PI * 2.0).cos() * r,
            (i as f32 / PARTICLE_COUNT as f32 * PI * 2.0).sin() * r,
        );

        let particle : tree::particle::Particle = tree::particle::Particle {
            acceleration : Vec2::new(0.0, 0.0),
            radius : random_f32() * random_f32() * (PARTICLE_MAX_RADIUS - PARTICLE_MIN_RADIUS) + PARTICLE_MIN_RADIUS,
            position : position,
            velocity : velocity
        };

        particles.push(particle);

        particles_indices.push(i);
        particles_tracer.push(0);
    }

    println!("lengths : {}", particles.len());

    let root = tree::Cell {
        center : Vec2::new(0.0, 0.0),
        size : Vec2::new(w, h),
        depth : 0,
        child_a : None, 
        child_b : None,
        start : 0,
        end : PARTICLE_COUNT,
        dimension : 0,
    };

    Model {
        particles : particles.to_vec(), 
        particles_indices : particles_indices.to_vec(), 
        particles_tracer: particles_indices.to_vec(), 
        root: root, 
        i : 0
    }
}

fn update(app: &App, model: &mut Model, update: Update) {

    let dt : f32 = (update.since_last.subsec_millis() as f32) * 0.001;

    //println!("fps: {}", 1.0 / dt);

    let window = app.main_window();
    let win = window.rect();
    let h = win.h();
    let w = win.w();

    model.root.size = Vec2::new(w, h);

    if model.i % 3 == 0 {
        model.root.split(&mut model.particles, &mut model.particles_indices, 10);
    }
    
    for i in 0..model.particles_indices.len() {
        model.particles_tracer[model.particles_indices[i]] = i;
    }

    for i in 0..model.particles.len() {
        let cells_near_particle = 
            model.root.ballwalk(model.particles[i].position, model.particles[i].radius + PARTICLE_MAX_RADIUS);

        let mut acceleration = Vec2::new(0.0, 0.0);
        for cell in cells_near_particle.iter() {
            let particles_in_cell : &[tree::particle::Particle] = &model.particles[cell.start..cell.end];
    
            for other_particle in particles_in_cell {
                let mut vector = other_particle.position.clone() - model.particles[i].position;
                let d = vector.length();

                // Skip self position
                if d > 0.01 && d < model.particles[i].radius + other_particle.radius {
                    vector = vector / d / d;

                    acceleration += 3000.0 * -vector;  
                }
            }
        }
        model.particles[i].acceleration = acceleration;

        let index = model.particles_indices[i];
        let vector = 
            model.particles[model.particles_tracer[(index+1) % model.particles.len()]].position.clone() - 
            model.particles[i].position;
        let d = vector.length();

        model.particles[i].acceleration += 500.0 * vector / d;
        
    }

    for particle in model.particles.iter_mut() {
        particle.kick_drift_kick(0.01);
        particle.enforce_boundary_conditions(w, h);
        // Damping as no energy conversion in system
        particle.velocity *= 0.95;
    }

    model.i += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // set background to blue
    frame.clear(BLACK);

    let n = model.particles.len();
    for i in 0..model.particles.len() {
        draw.ellipse()
        .color(WHITE)
        .x_y(model.particles[i].position.x, model.particles[i].position.y)
        .radius(2.0)
        .resolution(32.0);

        draw.line()
        .start(pt2(model.particles[model.particles_tracer[i]].position.x, model.particles[model.particles_tracer[i]].position.y))
        .end(pt2(model.particles[model.particles_tracer[(i+1)%n]].position.x, model.particles[model.particles_tracer[(i+1)%n]].position.y))
        .weight(1.0)
        .color(WHITE);
    
    }

    /*let mouse_pos = Vec2::new(app.mouse.x, app.mouse.y);
    let cells_near_mouse = model.root.ballwalk(mouse_pos, 10.0);

    for cell in cells_near_mouse.iter() {
        draw.rect()
        .color(ORANGERED)
        .x_y(cell.center.x, cell.center.y)
        .w(cell.size.x)
        .h(cell.size.y);

        let particles_in_cell : &[tree::particle::Particle] = &model.particles[cell.start..cell.end];

        for particle in particles_in_cell {
            draw.ellipse()
            .color(WHITE)
            .x_y(particle.position.x, particle.position.y)
            .radius(particle.radius)
            .resolution(32.0);
        }

    }*/

    //recursive_cell_view(&model.root, &draw);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join("out")
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}

fn recursive_cell_view(cell : &tree::Cell, draw : &Draw) {

    draw.rect()
    .no_fill()
    .stroke(GREY)
    .stroke_weight(0.8)
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
