use nannou::prelude::*;
use nannou::glam::Vec2;
pub mod particle;

type Link = Option<Box<Cell>>;

pub struct Cell {
    pub center : Vec2,
    pub size : Vec2,
    pub depth : i32,
    pub child_a : Link,
    pub child_b : Link,
}

impl Cell {
    // Split domain
    pub fn split(&mut self, particles : &mut[particle::Particle], max_depth : i32){
        let n = particles.len();

        if self.depth == max_depth {
            return;
        }

        let dimension = (self.size[0] < self.size[1]) as usize;
        let half_count : i32  = (n as i32) / 2;
        let mut step : f32 = self.size[dimension] / 2.0;
        let mut split : f32 = self.center[dimension];
        
        println!("{},{},{},{}", dimension, half_count, step, split);

        let left_count = loop {
            let mut left_count = 0;

            for (_i, particle) in particles.iter().enumerate() {
                left_count += (particle.position[dimension] < split ) as i32;
            }
            
            // maybe swithc to parallel version
            //particles.par_iter().filter(|&p| p.position[dimension] < split).reduce(|x, y| x + y);

            if abs(left_count - half_count) <= 1 { break left_count; }

            step /= 2.0;

            split += if left_count < half_count { step } else { -step };

            //println!("{}", left_count);
        };
        
        // Reshuffle array
        let mut i = 0;
        let mut j = n - 1;

        loop {
            if i == j { break ;}

            println!("{},{}", i, j);

            if particles[i].position[dimension] < split {
                i += 1;
                continue;
            }
            if particles[j].position[dimension] > split {
                j -= 1;
                continue;
            }

            particles.swap(i, j);
        }

        // Define new child cells
        let mut center_a : Vec2 = Vec2::new(0.0, 0.0);
        let mut center_b : Vec2 = Vec2::new(0.0, 0.0);

        center_a[1 - dimension] = self.center[1 - dimension];
        center_b[1 - dimension] = self.center[1 - dimension];

        let left = self.center[dimension] - self.size[dimension] / 2.0;
        let right = self.center[dimension] + self.size[dimension] / 2.0;

        let size_left = split - left;
        let size_right = right - split;

        center_a[dimension] = split - size_left / 2.0;
        center_b[dimension] = split + size_right / 2.0;

        let mut size_a : Vec2 = self.center.clone();
        let mut size_b : Vec2 = center_b.clone();

        size_a[dimension] = size_left;
        size_b[dimension] = size_right;

        let a = Box::new(Cell {
            center : center_a,
            size : size_a,
            depth : self.depth + 1,
            child_a : None,
            child_b : None
        });

        let b = Box::new(Cell {
            center : center_b,
            size : size_b,
            depth : self.depth + 1,
            child_a : None,
            child_b : None
        });

        self.child_a = Some(a);
        self.child_b = Some(b);

        match &mut self.child_a {
            Some(x) => x.split(&mut particles[0 .. left_count as usize], max_depth),
            None => ()
        }
        
        match &mut self.child_b {
            Some(x) => x.split(&mut particles[left_count as usize .. n], max_depth),
            None => ()
        }   
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics() {
        const COUNT : usize = 1<<5;

        let p = Vec2::new(-100.0, 0.0);
        let v = Vec2::new(0.0, 0.0);

        let mut particles : [particle::Particle; COUNT] = [particle::Particle{position : p, velocity : v}; COUNT];

        for i in COUNT/2..COUNT {
            particles[i].position.x = 100.0;
        }

        let mut cell : Cell = Cell {
            center : Vec2::new(0.0, 0.0),
            size : Vec2::new(100.0, 50.0),
            depth : 0,
            child_a : None,
            child_b : None
        };

        cell.split(&mut particles, 2);

        let x = cell.center.x;
        assert_eq!(x, 0.0);

    }
}