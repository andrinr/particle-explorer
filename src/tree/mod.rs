use nannou::prelude::*;
use nannou::glam::Vec2;
pub mod particle;

type Link = Option<Box<Cell>>;

const RATIO_CHANGE_TRHESHOLD : f32 = 0.2;

pub struct Cell {
    pub center : Vec2,
    pub size : Vec2,
    pub depth : i32,
    pub child_a : Link,
    pub child_b : Link,
    pub start : usize,
    pub end : usize,
    pub dimension : usize
}

impl Cell {

    pub fn split(&mut self, particles : &mut[particle::Particle], indices : &mut[usize], max_depth : i32){
        let n = particles.len();

        if self.depth == max_depth || n < 2 {
            return;
        }

        let ratio = self.size.x / self.size.y;
        let mut dimension = (self.size.x < self.size.y) as usize;

        if abs(1.0 - ratio) < RATIO_CHANGE_TRHESHOLD { 
            dimension = self.dimension;
        }

        self.dimension = dimension;

        let half_count : i32  = (n as i32) / 2;
        let mut step : f32 = self.size[dimension] / 2.0;
        let mut split : f32 = self.center[dimension];
        
        let mut i = 0;
        let left_count = loop {
            let mut counter = 0;

            for (_i, particle) in particles.iter().enumerate() {
                counter += (particle.position[dimension] < split ) as i32;
            }

            // maybe swithc to parallel version
            //particles.par_iter_mut().filter(|&p| p.position[dimension] < split).reduce(|x, y| x + y);

            i = i + 1;
            if abs(counter - half_count) <= 1 { break counter; }

            step /= 2.0;

            split += if counter < half_count { step } else { -step };

        };
        // Reshuffle array
        let mut i = 0;
        let mut j = n - 1;

        loop {
            if i == j { break ;}

            //println!("{},{}", i, j);

            if particles[i].position[dimension] < split {
                i += 1;
                continue;
            }
            if particles[j].position[dimension] > split {
                j -= 1;
                continue;
            }

            particles.swap(i, j);

            indices.swap(i,j);
        }

        // Define new child cells
        let mut center_a : Vec2 = Vec2::new(0.0, 0.0);
        let mut center_b : Vec2 = Vec2::new(0.0, 0.0);

        center_a[1 - dimension] = self.center[1 - dimension];
        center_b[1 - dimension] = self.center[1 - dimension];

        let left = self.center[dimension] - self.size[dimension] / 2.0;
        let right = self.center[dimension] + self.size[dimension] / 2.0;

        let size_left_child = split - left;
        let size_rigth_child = right - split;

        center_a[dimension] = split - size_left_child / 2.0;
        center_b[dimension] = split + size_rigth_child / 2.0;

        let mut size_a : Vec2 = self.size.clone();
        let mut size_b : Vec2 = self.size.clone();

        size_a[dimension] = size_left_child;
        size_b[dimension] = size_rigth_child;

        // Create or update child cells
        match &mut self.child_a {
            Some(x) => {
                x.center = center_a;
                x.size = size_a;
                x.start = self.start;
                x.end = self.start + left_count as usize;
                x.depth = self.depth + 1;
            },
            None => {
                self.child_a = Some(Box::new(Cell {
                    center : center_a,
                    size : size_a,
                    depth : self.depth + 1,
                    child_a : None,
                    child_b : None,
                    start : self.start,
                    end : self.start + left_count as usize,
                    dimension : 0,
                }));
            }
        }

        match &mut self.child_b {
            Some(x) => {
                x.center = center_b;
                x.size = size_b;
                x.start = self.start + left_count as usize;
                x.end = self.end;
                x.depth = self.depth + 1;
            },
            None => {
                self.child_b = Some(Box::new(Cell {
                    center : center_b,
                    size : size_b,
                    depth : self.depth + 1,
                    child_a : None,
                    child_b : None,
                    start : self.start + left_count as usize,
                    end : self.end,
                    dimension : 0,
                }));
            }
        }   

        match &mut self.child_a {
            Some(x) => x.split(
                &mut particles[0 .. left_count as usize],
                &mut indices[0 .. left_count as usize],
                max_depth),
            None => {}
        } 

        match &mut self.child_b {
            Some(x) => x.split(
                &mut particles[left_count as usize .. n], 
                &mut indices[left_count as usize .. n],
                max_depth),
            None => {}
        }   
    }

    // Todo add periodic boundaries
    pub fn ballwalk(&self, pos : Vec2, radius : f32) -> Vec<&Cell> {

        let left = self.center.x - self.size.x / 2.0;
        let right = self.center.x + self.size.x / 2.0;

        let bottom = self.center.y - self.size.y / 2.0;
        let top = self.center.y + self.size.y / 2.0;

        let within = 
            pos.x > left - radius && 
            pos.x < right + radius && 
            pos.y > bottom - radius && 
            pos.y < top + radius;

        let mut res : Vec<&Cell> = Vec::new();

        if !within {
            return res;
        }

        match &self.child_a {
            Some(x) => res.append(&mut x.ballwalk(pos, radius)),
            None => ()
        };
        
        match &self.child_b {
            Some(x) => res.append(&mut x.ballwalk(pos, radius)),
            None => ()
        };

        if !self.child_a.is_some() {
            res.push(self)
        }
    
        return res;
    }
}