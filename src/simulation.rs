use crate::bird::Bird;
use crate::vector2d::Vec2D;
use crate::flock::{FlockParameters, calculate_acceleration};
use crate::spatial_grid::SpatialGrid;

use ::rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use std::time::Instant;

pub struct Simulation {
    birds: Vec<Bird>,
    params: FlockParameters,
    dt: f64,
    accumulator: f64,
    accelerations: Vec<Vec2D>,
    grid: SpatialGrid,
    rng: rand::rngs::ThreadRng,
    flock_destination: Vec2D,
    destination_timer: f64,
}

impl Simulation {
    pub fn birds(&self) -> &[Bird] {
        &self.birds
    }
    pub fn params(&self) -> &FlockParameters {
        &self.params
    }
    pub fn params_mut(&mut self) -> &mut FlockParameters {
        &mut self.params
    }
    pub fn dt(&self) -> f64 {
        self.dt
    }
    pub fn advance(&mut self, frame_time: f64) {
        self.accumulator += frame_time.min(0.1);
        while self.accumulator >= self.dt {
            self.step();
            self.accumulator -= self.dt;
        }
    }
    pub fn new(birds: Vec<Bird>, dt: f64, params: FlockParameters) -> Self {
        let grid = SpatialGrid::new(params.sim_width,params.sim_height,params.cell_size);
        let accelerations = Vec::with_capacity(birds.len());
        let mut rng = rand::rng();
        let flock_destination = Vec2D { x: rng.random_range(0.0..params.sim_width),y: rng.random_range(0.0..params.sim_height),}; 
        Simulation {birds,params,dt,accumulator:0.0,accelerations,grid,rng,flock_destination,destination_timer:0.0}        
    }
    pub fn new_random(flock_size: usize, dt: f64, params: FlockParameters) -> Self {
        let mut rng = rand::rng();
        let mut birds: Vec<Bird> = Vec::with_capacity(flock_size);
        for _ in 0.. flock_size {
            let position = Vec2D {x: rng.random_range(0.0..=params.sim_width),y: rng.random_range(0.0..=params.sim_height)};
            let velocity = Vec2D {x: rng.random_range(-params.max_speed..=params.max_speed),y: rng.random_range(-params.max_speed..=params.max_speed)};
            birds.push(Bird::new(position, velocity));            
        }

        Self::new(birds,dt,params)   

    }
    pub fn new_random_seeded(flock_size: usize, dt: f64, params: FlockParameters, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut birds: Vec<Bird> = Vec::with_capacity(flock_size);
        for _ in 0.. flock_size {
            let position = Vec2D {x: rng.random_range(0.0..=params.sim_width),y: rng.random_range(0.0..=params.sim_height)};
            let velocity = Vec2D {x: rng.random_range(-params.max_speed..=params.max_speed),y: rng.random_range(-params.max_speed..=params.max_speed)};
            birds.push(Bird::new(position, velocity));             
        }

        Self::new(birds,dt,params)   

    }
    pub fn add_bird(&mut self, bird: Bird) {
        self.birds.push(bird);
    }
    pub fn remove_bird(&mut self, index: usize) -> Option<Bird> {
        if index < self.birds.len() {
            Some(self.birds.swap_remove(index))
        } else {
            None
        }
    }
    pub fn flock_size(&self) -> usize {
        self.birds.len()
    }
    pub fn update_destination(&mut self) {
        self.destination_timer += self.dt;

        if self.destination_timer >= self.params.destination_interval {
            self.flock_destination = Vec2D {x: self.rng.random_range(0.0..self.params.sim_width),y: self.rng.random_range(0.0..self.params.sim_height),};
            self.destination_timer -= self.params.destination_interval;
        }

    }

    pub fn step(&mut self) {
        self.update_destination();
        self.grid.rebuild(&self.birds);
        let count = self.birds.len();
        self.accelerations.clear();
        for index in 0..count { // first find each bird neighbors and calculate acceleration
            let acceleration = calculate_acceleration(&self.birds,index,&self.params,&self.grid,&self.flock_destination);
            self.accelerations.push(acceleration);               
        } 
        for (index,bird) in self.birds.iter_mut().enumerate() { // next update the birds using accelerations
            bird.update(&self.accelerations[index],self.dt,self.params.max_speed,self.params.sim_width,self.params.sim_height);
        }          
    }
    pub fn benchmark(&mut self, steps:usize) -> f64 {
        let start = Instant::now();
        for _ in 0..steps {
            self.step();
        }
        let elapsed = start.elapsed();
        let milliseconds = elapsed.as_secs_f64()*1000.0;
        let avg_exec_time = milliseconds / steps as f64;
        avg_exec_time
    }
}