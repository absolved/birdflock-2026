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
    rng: StdRng,
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
        let tolerance = 1e-12* self.dt;
        self.accumulator += frame_time.min(0.1);
        while self.accumulator + tolerance >= self.dt {
            self.step();
            self.accumulator = (self.accumulator - self.dt).max(0.0);
        }
    }
    fn new_with_rng(birds: Vec<Bird>, dt: f64, params: FlockParameters, mut rng: StdRng) -> Self{
        let grid = SpatialGrid::new(
            params.sim_width,
            params.sim_height,
            params.cell_size,
        );

        let accelerations = Vec::with_capacity(birds.len());

        let flock_destination = Vec2D {
            x: rng.random_range(0.0..params.sim_width),
            y: rng.random_range(0.0..params.sim_height),
        };

        Self {
            birds,
            params,
            dt,
            accumulator: 0.0,
            accelerations,
            grid,
            rng,
            flock_destination,
            destination_timer: 0.0,
        }
    }
    pub fn new(birds: Vec<Bird>, dt: f64, params: FlockParameters) -> Self {
        let rng = StdRng::from_rng(&mut rand::rng());
        Self::new_with_rng(birds, dt, params, rng)      
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

        Self::new_with_rng(birds, dt, params, rng)   

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
        const TIME_EPSILON: f64 = 1e-12;
        self.destination_timer += self.dt;
        let interval = self.params.destination_interval;

        if self.destination_timer + TIME_EPSILON >= interval {
            self.flock_destination = Vec2D {x: self.rng.random_range(0.0..self.params.sim_width),y: self.rng.random_range(0.0..self.params.sim_height),};
            let remainder = self.destination_timer.rem_euclid(interval);
            self.destination_timer = if remainder >= interval - TIME_EPSILON {
                0.0
            } else {
                remainder
            };
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

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 1e-12;

    fn test_params() -> FlockParameters {
        FlockParameters {
            separation_gain: 15.0,
            cohesion_gain: 0.01,
            alignment_gain: 1.0,
            global_cohesion_gain : 0.1,
            neighbor_radius: 10.0,
            separation_radius: 8.0,
            collision_radius: 2.0,
            max_speed: 10.0,
            alignment_speed_gain: 1.0,
            sim_width: 100.0,
            sim_height: 100.0,
            cell_size: 10.0,
            bird_size: 10.0,
            destination_interval: 1.0,
            separation_softening: 0.1,
        }
    }

    #[test]
    fn test_step_constant_velocity() {
        let params = FlockParameters {
            separation_gain: 0.0,
            cohesion_gain: 0.0,
            alignment_gain: 0.0,
            global_cohesion_gain: 0.0,
            ..test_params()
        };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 20.0 }, Vec2D { x: 3.0, y: 4.0}),
        ];

        let dt = 0.1;
        let mut simulation = Simulation::new(birds, dt, params);
        simulation.step();
        let bird = &simulation.birds[0];
        let p = bird.position();
        let v = bird.velocity();
        let expected_vx = 3.0;
        let expected_vy = 4.0;
        let expected_px = 10.3;
        let expected_py = 20.4;
        assert!((p.x-expected_px).abs() < EPSILON, "Expected bird.position.x = {} got {}", expected_px, p.x);
        assert!((p.y-expected_py).abs() < EPSILON, "Expected bird.position.y = {} got {}", expected_py, p.y);
        assert!((v.x-expected_vx).abs() < EPSILON, "Expected bird.velocity.x = {} got {}", expected_vx, v.x);
        assert!((v.y-expected_vy).abs() < EPSILON, "Expected bird.velocity.y = {} got {}", expected_vy, v.y);
    }

    #[test]
    fn test_step_acceleration_integration() {
        let params = FlockParameters {
            separation_gain: 0.0,
            cohesion_gain: 0.0,
            alignment_gain: 0.0,
            global_cohesion_gain: 0.5,
            ..test_params()
        };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 20.0 }, Vec2D { x: 3.0, y: 4.0}),
        ];

        let dt = 0.1;
        let mut simulation = Simulation::new(birds, dt, params);
        simulation.flock_destination = Vec2D { x: 20.0, y: 30.0};
        simulation.step();
        let bird = &simulation.birds[0];
        let p = bird.position();
        let v = bird.velocity();
        let expected_vx = 3.5;
        let expected_vy = 4.5;
        let expected_px = 10.35;
        let expected_py = 20.45;
        assert!((p.x-expected_px).abs() < EPSILON, "Expected bird.position.x = {} got {}", expected_px, p.x);
        assert!((p.y-expected_py).abs() < EPSILON, "Expected bird.position.y = {} got {}", expected_py, p.y);
        assert!((v.x-expected_vx).abs() < EPSILON, "Expected bird.velocity.x = {} got {}", expected_vx, v.x);
        assert!((v.y-expected_vy).abs() < EPSILON, "Expected bird.velocity.y = {} got {}", expected_vy, v.y);
    }

    #[test]
    fn test_step_simultaneous_updates() {
        let params = FlockParameters {
            separation_gain: 1.0,
            cohesion_gain: 0.0,
            alignment_gain: 0.0,
            global_cohesion_gain: 0.0,
            ..test_params()
        };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 10.0 }, Vec2D { x: 0.0, y: 0.0}),
            Bird::new(Vec2D { x: 14.0, y: 10.0 }, Vec2D { x: 0.0, y: 0.0}),
        ];

        let dt = 0.1;
        let mut simulation = Simulation::new(birds, dt, params);
        simulation.flock_destination = Vec2D { x: 20.0, y: 30.0};
        simulation.step();
        let bird_0 = &simulation.birds[0];
        let bird_1 = &simulation.birds[1];
        let p_0 = bird_0.position();
        let v_0 = bird_0.velocity();
        let p_1 = bird_1.position();
        let v_1 = bird_1.velocity();
        let s = (4.0_f64 / 4.1).powi(2);
        let expected_vx_0 = -0.1*s;
        let expected_vy_0 = 0.0;
        let expected_px_0 = 10.0 - 0.01*s;
        let expected_py_0 = 10.0;
        let expected_vx_1 = 0.1*s;
        let expected_vy_1 = 0.0;
        let expected_px_1 = 14.0 + 0.01*s;
        let expected_py_1 = 10.0;
        assert!((p_0.x-expected_px_0).abs() < EPSILON, "Expected bird_0.position.x = {} got {}", expected_px_0, p_0.x);
        assert!((p_0.y-expected_py_0).abs() < EPSILON, "Expected bird_0.position.y = {} got {}", expected_py_0, p_0.y);
        assert!((v_0.x-expected_vx_0).abs() < EPSILON, "Expected bird_0.velocity.x = {} got {}", expected_vx_0, v_0.x);
        assert!((v_0.y-expected_vy_0).abs() < EPSILON, "Expected bird_0.velocity.y = {} got {}", expected_vy_0, v_0.y);
        assert!((p_1.x-expected_px_1).abs() < EPSILON, "Expected bird_1.position.x = {} got {}", expected_px_1, p_1.x);
        assert!((p_1.y-expected_py_1).abs() < EPSILON, "Expected bird_1.position.y = {} got {}", expected_py_1, p_1.y);
        assert!((v_1.x-expected_vx_1).abs() < EPSILON, "Expected bird_1.velocity.x = {} got {}", expected_vx_1, v_1.x);
        assert!((v_1.y-expected_vy_1).abs() < EPSILON, "Expected bird_1.velocity.y = {} got {}", expected_vy_1, v_1.y);
    }

    #[test]
    fn test_destination_update_timing() {
        let params = FlockParameters {
            destination_interval: 1.0,
            ..test_params()
        };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 10.0 }, Vec2D { x: 0.0, y: 0.0}),
            Bird::new(Vec2D { x: 14.0, y: 10.0 }, Vec2D { x: 0.0, y: 0.0}),
        ];

        let dt = 0.1;
        let mut simulation = Simulation::new(birds, dt, params);
        let initial_destination = simulation.flock_destination;
        for _ in 0..9 {
            simulation.update_destination();
        }
        assert!((simulation.flock_destination.x-initial_destination.x).abs() < EPSILON, "Expected simulation.flock_destination.x = {} got {}", initial_destination.x, simulation.flock_destination.x);
        assert!((simulation.flock_destination.y-initial_destination.y).abs() < EPSILON, "Expected simulation.flock_destination.y = {} got {}", initial_destination.y, simulation.flock_destination.y);
        assert!((simulation.destination_timer-0.9).abs() < EPSILON, "Expected simulation.destination_timer = 0.9 got {}", simulation.destination_timer);
        simulation.update_destination();
        assert!((simulation.destination_timer-0.0).abs() < EPSILON, "Expected simulation.destination_timer = 0.0 got {}", simulation.destination_timer);
        assert!((simulation.flock_destination.x <= simulation.params.sim_width), "Expected simulation.flock_destination.x <= {} got {}", simulation.params.sim_width, simulation.flock_destination.x);
        assert!((simulation.flock_destination.x >= 0.0), "Expected simulation.flock_destination.x >= 0.0 got {}", simulation.flock_destination.x);
        assert!((simulation.flock_destination.y <= simulation.params.sim_height), "Expected simulation.flock_destination.y <= {} got {}", simulation.params.sim_height, simulation.flock_destination.y);
        assert!((simulation.flock_destination.y >= 0.0), "Expected simulation.flock_destination.y >= 0.0 got {}", simulation.flock_destination.y);

    }

    #[test]
    fn test_destination_update_large_dt() {
        let params = FlockParameters {
            destination_interval: 0.1,
            ..test_params()
        };

        let birds = vec![
            Bird::new(
                Vec2D { x: 10.0, y: 10.0 },
                Vec2D { x: 0.0, y: 0.0 },
            ),
        ];
        let dt = 0.35;
        let mut simulation = Simulation::new(birds, dt, params);
        simulation.update_destination();
        assert!((simulation.destination_timer - 0.05).abs() < EPSILON,"Expected remaining time = 0.05, got {}",simulation.destination_timer);        
    }

    #[test]
    fn test_seeded_simulation_reproducibility() {
        let params_a = FlockParameters {destination_interval: 0.2,..test_params()};
        let params_b = FlockParameters {destination_interval: 0.2,..test_params()};
        let seed = 42;
        let flock_size = 10;
        let dt = 0.1;
        let mut sim_a = Simulation::new_random_seeded(flock_size,dt,params_a,seed,);
        let mut sim_b = Simulation::new_random_seeded(flock_size,dt,params_b,seed,);
        for _ in 0..21 {
            assert!((sim_a.flock_destination.x - sim_b.flock_destination.x).abs() < EPSILON);
            assert!((sim_a.flock_destination.y - sim_b.flock_destination.y).abs() < EPSILON);
            assert_eq!(sim_a.birds.len(), sim_b.birds.len());
            for (bird_a, bird_b) in sim_a.birds.iter().zip(sim_b.birds.iter()) {
                let pa = bird_a.position();
                let pb = bird_b.position();

                assert!((pa.x - pb.x).abs() < EPSILON);
                assert!((pa.y - pb.y).abs() < EPSILON);

                let va = bird_a.velocity();
                let vb = bird_b.velocity();

                assert!((va.x - vb.x).abs() < EPSILON);
                assert!((va.y - vb.y).abs() < EPSILON);
            }
            sim_a.step();
            sim_b.step();    
        }
    }

    #[test]
    fn test_different_seeds() {
        let flock_size = 10;
        let dt = 0.1;
        let sim_a = Simulation::new_random_seeded(flock_size,dt,test_params(),42);
        let sim_b = Simulation::new_random_seeded(flock_size,dt,test_params(),123);
        
        let different = sim_a.birds.iter().zip(sim_b.birds.iter()).any(|(a,b)| {
                let pa = a.position();
                let pb = b.position();
                let va = a.velocity();
                let vb = b.velocity();
                ((pa.x - pb.x).abs() > EPSILON) ||
                ((pa.y - pb.y).abs() > EPSILON) ||
                ((va.x - vb.x).abs() > EPSILON) ||
                ((va.y - vb.y).abs() > EPSILON)                
                                                                  });
        assert!(different);                                                          
    }

    #[test]
    fn test_advance_variable_frame_times() {
        let params = FlockParameters {
            separation_gain: 0.0,
            cohesion_gain: 0.0,
            alignment_gain: 0.0,
            global_cohesion_gain: 0.0,
            ..test_params()
        };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 20.0}, Vec2D { x: 3.0, y: 4.0}),
        ];

        let dt = 0.01;
        let mut simulation = Simulation::new(birds, dt, params);
        simulation.advance(0.025);
        let p = simulation.birds[0].position();
        assert!((p.x-10.06).abs() < EPSILON, "Expected bird.position.x = 10.06 got {}", p.x);
        assert!((p.y-20.08).abs() < EPSILON, "Expected bird.position.y = 20.08 got {}", p.y);
        assert!((simulation.accumulator - 0.005).abs() < EPSILON, "Expected simulation.accumulator = 0.005 got {}", simulation.accumulator);
        simulation.advance(0.004);
        let p = simulation.birds[0].position();
        assert!((p.x-10.06).abs() < EPSILON, "Expected bird.position.x = 10.06 got {}", p.x);
        assert!((p.y-20.08).abs() < EPSILON, "Expected bird.position.y = 20.08 got {}", p.y);
        assert!((simulation.accumulator - 0.009).abs() < EPSILON, "Expected simulation.accumulator = 0.009 got {}", simulation.accumulator);
        simulation.advance(0.006);
        let p = simulation.birds[0].position();
        assert!((p.x-10.09).abs() < EPSILON, "Expected bird.position.x = 10.09 got {}", p.x);
        assert!((p.y-20.12).abs() < EPSILON, "Expected bird.position.y = 20.12 got {}", p.y);
        assert!((simulation.accumulator - 0.005).abs() < EPSILON, "Expected simulation.accumulator = 0.005 got {}", simulation.accumulator);
    }

    #[test]
    fn test_advance_frame_time_clamp() {
        let params = FlockParameters {
            separation_gain: 0.0,
            cohesion_gain: 0.0,
            alignment_gain: 0.0,
            global_cohesion_gain: 0.0,
            ..test_params()
        };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 20.0}, Vec2D { x: 3.0, y: 4.0}),
        ];

        let dt = 0.01;
        let mut simulation = Simulation::new(birds, dt, params);
        let expected_x = 10.3;
        let expected_y = 20.4;
        simulation.advance(0.5);
        let p = simulation.birds[0].position();
        assert!((p.x-expected_x).abs() < EPSILON, "Expected bird.position.x = {} got {}", expected_x, p.x);
        assert!((p.y-expected_y).abs() < EPSILON, "Expected bird.position.y = {} got {}", expected_y, p.y);
        assert!((simulation.accumulator).abs() < EPSILON, "Expected simulation.accumulator = 0.0 got {}", simulation.accumulator);
    }

    #[test]
    fn test_advance_accumulation_boundary() {
        let params = FlockParameters {
            separation_gain: 0.0,
            cohesion_gain: 0.0,
            alignment_gain: 0.0,
            global_cohesion_gain: 0.0,
            ..test_params()
        };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 20.0}, Vec2D { x: 3.0, y: 4.0}),
        ];

        let dt = 0.1;
        let mut simulation = Simulation::new(birds, dt, params);
        let expected_x = 10.3;
        let expected_y = 20.4;
        for _ in 0..10 {
            simulation.advance(0.01);
        }
        
        let p = simulation.birds[0].position();
        assert!((p.x-expected_x).abs() < EPSILON, "Expected bird.position.x = {} got {}", expected_x, p.x);
        assert!((p.y-expected_y).abs() < EPSILON, "Expected bird.position.y = {} got {}", expected_y, p.y);
        assert!((simulation.accumulator).abs() < EPSILON, "Expected simulation.accumulator = 0.0 got {}", simulation.accumulator);
    }

    #[test]
    fn test_simulation_invariants() {
        const EPSILON: f64 =  1e-12;
        let params = test_params();
        let max_speed = params.max_speed;
        let width = params.sim_width;
        let height = params.sim_height;

        let mut simulation = Simulation::new_random_seeded(50,0.01, params, 42);

        for step in 0..1000 {
            simulation.step();

            for (index, bird) in simulation.birds.iter().enumerate() {
                let p = bird.position();
                let v = bird.velocity();
                assert!(p.x < width, "Expected bird.position.x < {} got {} for bird {} on step {}", width, p.x, index, step);
                assert!(p.x >= 0.0, "Expected bird.position.x >= 0.0 got {} for bird {} on step {}", p.x, index, step);
                assert!(p.y < height, "Expected bird.position.y < {} got {} for bird {} on step {}", height, p.y, index, step);
                assert!(p.y >= 0.0, "Expected bird.position.y >= 0.0 got {} for bird {} on step {}", p.y, index, step);
                assert!(v.magnitude() <= max_speed + EPSILON,"Expected bird.velocity.magnitude <= {} got {} for bird {} on step {}", max_speed, v.magnitude(), index, step);
            }
        }
    }

    #[test]
    fn test_simulation_extreme_gains() {
        const EPSILON: f64 =  1e-12;
        let params = FlockParameters {
            separation_gain: 1000.0,
            cohesion_gain: 100.0,
            alignment_gain: 100.0,
            global_cohesion_gain: 100.0,
            ..test_params()
        };
        let max_speed = params.max_speed;
        let width = params.sim_width;
        let height = params.sim_height;

        let mut simulation = Simulation::new_random_seeded(50,0.01, params, 42);
        let mut saturated_count = 0;
        for step in 0..1000 {
            simulation.step();

            for (index, bird) in simulation.birds.iter().enumerate() {
                let p = bird.position();
                let v = bird.velocity();
                let speed = v.magnitude();
                if (speed - max_speed).abs() < 1e-9 {
                    saturated_count += 1;
                }
                assert!(p.x < width, "Expected bird.position.x < {} got {} for bird {} on step {}", width, p.x, index, step);
                assert!(p.x >= 0.0, "Expected bird.position.x >= 0.0 got {} for bird {} on step {}", p.x, index, step);
                assert!(p.y < height, "Expected bird.position.y < {} got {} for bird {} on step {}", height, p.y, index, step);
                assert!(p.y >= 0.0, "Expected bird.position.y >= 0.0 got {} for bird {} on step {}", p.y, index, step);
                assert!(v.magnitude() <= max_speed + EPSILON,"Expected bird.velocity.magnitude <= {} got {} for bird {} on step {}", max_speed, v.magnitude(), index, step);
            }
        }
        let total_observations = simulation.birds.len() * 1000;
        let saturation_ratio = saturated_count as f64 / total_observations as f64;
        println!("The saturation ratio is {}",saturation_ratio);
    }                   
}