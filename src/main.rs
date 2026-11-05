mod rendering;
use birdflock::simulation::Simulation;
use birdflock::flock::FlockParameters;
use rendering::{draw_birds,draw_hud};

use macroquad::prelude::*;

#[macroquad::main("bird flock Sim")]
async fn main() {

    let params = FlockParameters {
        separation_gain: 1.5,
        cohesion_gain: 0.05,
        alignment_gain: 0.5,
        neighbor_radius: 5.0,
        max_speed: 50.0,
        sim_width: 100.0,
        sim_height: 100.0,
        cell_size: 5.0,
        bird_size: 10.0,
    };
    let flock_size = 100;
    let dt = 0.01;
    let mut simulation = Simulation::new_random(flock_size, dt, params);
    // simulation.grid.rebuild(&simulation.birds());
    // verify_neighbors(&simulation.birds, &simulation.grid, simulation.params.neighbor_radius, simulation.params.width, simulation.params.height);
    // let sim_flocksizes = [50,100,200,400,800];
    // for flocksize in sim_flocksizes {
    //     simulation.birds = create_flock(flocksize,simulation.params.max_speed,simulation.params.width,simulation.params.height);
    //     let avg_exec_time = simulation.benchmark(100);
    //     println!("Average step time for flock of size {:?} = {:?} ms ",flocksize,avg_exec_time);
    // }

    loop {
        if is_key_pressed(KeyCode::Q) {
            simulation.params_mut().separation_gain += 0.1;
        }
        if is_key_pressed(KeyCode::A) {
            simulation.params_mut().separation_gain = (simulation.params().separation_gain - 0.1).max(0.0);
        }
        if is_key_pressed(KeyCode::W) {
            simulation.params_mut().cohesion_gain += 0.01;
        }
        if is_key_pressed(KeyCode::S) {
            simulation.params_mut().cohesion_gain = (simulation.params().cohesion_gain - 0.01).max(0.0);
        }
        if is_key_pressed(KeyCode::E) {
            simulation.params_mut().alignment_gain += 0.1;
        }
        if is_key_pressed(KeyCode::D) {
            simulation.params_mut().alignment_gain = (simulation.params().alignment_gain - 0.1).max(0.0);
        }
        simulation.advance(get_frame_time() as f64);
        clear_background(BLACK);
        draw_birds(&simulation.birds(),&simulation.params());
        draw_hud(&simulation.params());
        next_frame().await;
    }
}