extern crate nannou;

mod tree;

use nannou::prelude::*;
use nannou::glam::Vec2;

const PARTICLE_COUNT : usize = 1<<12;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    particles : [tree::particle::Particle; PARTICLE_COUNT],
    root : tree::Cell,
    i : i32
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

    Model {particles : particles, root: root, i : 0}
}

fn update(app: &App, model: &mut Model, update: Update) {

    let _dt : f32 = (update.since_last.subsec_millis() as f32) * 0.001;

    let window = app.main_window();
    let win = window.rect();
    let h = win.h();
    let w = win.w();

    model.root.size = Vec2::new(w, h);


    /*for particle in model.particles.iter() {
        let cells_near_particle = model.root.ballwalk(particle.position, 2.0);

        for cell in cells_near_particle.iter() {
            let particles_in_cell : &[tree::particle::Particle] = &model.particles[cell.start..cell.end];
    
            for other_particle in particles_in_cell {
                let d = other_particle.position.distance(particle.position);
                if d < 2.0 {

                }
            }
        }
    }*/

    for particle in model.particles.iter_mut() {
        particle.kick_drift_kick(0.01, Vec2::new(0.0, 0.0));
        particle.enforce_boundary_conditions(w, h);
    }


    if model.i % 10 == 0 {
        model.root.split(&mut model.particles[0..PARTICLE_COUNT], 8);
    }
    model.i += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // set background to blue
    frame.clear(GREY);

    for particle in model.particles {
        draw.ellipse()
        .color(BLACK)
        .x_y(particle.position.x, particle.position.y)
        .radius(1.0)
        .resolution(8.0);
    }
    
    let mouse_pos = Vec2::new(app.mouse.x, app.mouse.y);
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
            .radius(2.0)
            .resolution(8.0);
        }

    }

    recursive_cell_view(&model.root, &draw);

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
    .stroke(BLACK)
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
