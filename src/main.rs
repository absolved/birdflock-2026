use macroquad::prelude::*;
use ::rand::RngExt;
use std::time::Instant;

#[derive(Debug)]
struct Bird {
    position: Vec2D,
    velocity: Vec2D,
}

impl Bird {
    fn update(&mut self, acceleration: &Vec2D, dt: f64, max_speed: f64, width: f64, height: f64) {
        self.velocity = self.velocity.add(&acceleration.scale(dt)).limit(max_speed);
        self.position = self.position.add(&self.velocity.scale(dt)).wrap(width,height);
    }
}

#[derive(Debug, Clone, Copy)]
struct Vec2D {
    x: f64,
    y: f64,
}

impl Vec2D {
    fn magnitude(&self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()    
    }

    fn subtract(&self, other: &Vec2D) -> Vec2D {
        Vec2D {
            x:self.x-other.x,
            y:self.y-other.y
        }
    }
    fn add(&self, other: &Vec2D) -> Vec2D {
        Vec2D {
            x:self.x+other.x,
            y:self.y+other.y
        }
    }
    fn scale(&self, scalar:f64) -> Vec2D {
        Vec2D {
            x:scalar*self.x,
            y:scalar*self.y
        }
    }
    fn limit(&self, max_magnitude: f64) -> Vec2D {
        let v_mag = self.magnitude();
        if v_mag > max_magnitude {
            self.scale(max_magnitude/v_mag)
        } else {
            *self
        }
    }
    fn wrap(&self, width: f64, height: f64) -> Vec2D {
        Vec2D {
            x: self.x.rem_euclid(width),
            y: self.y.rem_euclid(height)
        }
    }    
    fn periodic_displacement(&self, other: &Vec2D, width: f64, height: f64,) -> Vec2D {
        let displacement = other.subtract(self);
        let half_domain = Vec2D {
            x: width/2.0, y: height/2.0
        };
        Vec2D {
            x: (displacement.x+half_domain.x).rem_euclid(width) - half_domain.x,
            y: (displacement.y+half_domain.y).rem_euclid(height) - half_domain.y
        }
    }    
}

struct FlockParameters {
    separation_gain: f64,
    cohesion_gain: f64,
    alignment_gain: f64,
    neighbor_radius: f64,
    max_speed: f64,
    width: f64,
    height: f64,
    flock_size: usize,
    bird_size: f32,
}

struct Simulation {
    birds: Vec<Bird>,
    params: FlockParameters,
    dt: f64,
    accumulator: f64,
    accelerations: Vec<Vec2D>,
    grid: SpatialGrid,
}

impl Simulation {
    fn step(&mut self) {
        self.grid.rebuild(&self.birds);
        let count = self.birds.len();
        self.accelerations.clear();
        for index in 0..count { // first find each bird neighbors and calculate acceleration
            let acceleration = calculate_acceleration(&self.birds,index,&self.params,&self.grid);
            self.accelerations.push(acceleration);               
        } 
        for (index,bird) in self.birds.iter_mut().enumerate() { // next update the birds using accelerations
            bird.update(&self.accelerations[index],self.dt,self.params.max_speed,self.params.width,self.params.height);
        }          
    }
    fn benchmark(&mut self, steps:usize) -> f64 {
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

struct SpatialGrid { // divide simulation into a grid of square cells. cells is a list of lists, where the inner list is the birds occupying each cell.
    cells: Vec<Vec<usize>>,
    cols: usize, // number of cols in grid
    rows: usize, // number of rows in grid
    cell_size: f64, // width and height of each cell
}

impl SpatialGrid {
    fn new(width: f64, height: f64, cell_size: f64) -> Self {
        let cols = (width / cell_size).ceil() as usize;
        let rows = (height / cell_size).ceil() as usize;
        let cells = vec![Vec::<usize>::new(); cols*rows];
        SpatialGrid {
            cells,
            cols,
            rows,
            cell_size,
        }
    }
    fn rebuild(&mut self, birds: &[Bird]) { // update the neighbor information in each cell after birds move
        for cell in self.cells.iter_mut() {
            cell.clear();
        }
        for (bird_index,bird) in birds.iter().enumerate() {
            let (cx,cy) = cell_coordinates(&bird.position, self.cell_size);
            let index = cell_index(cx,cy, self.cols);
            self.cells[index].push(bird_index);
        }
    }
    fn neighboring_cells(&self, cx: usize, cy: usize) -> Vec<usize> {
        let mut neighbor_cells: Vec<usize> = Vec::new(); 
        for dy in -1..=1 {
            for dx in -1..=1 {
                let wrapped_x = (cx as isize + dx).rem_euclid(self.cols as isize) as usize;
                let wrapped_y = (cy as isize + dy).rem_euclid(self.rows as isize) as usize;
                neighbor_cells.push(cell_index(wrapped_x,wrapped_y,self.cols));
            }
        }
        neighbor_cells
    }
    fn find_neighbors(&self, birds: &[Bird], bird_index: usize, radius: f64, width: f64, height: f64,) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let bird = &birds[bird_index];
        let (cx,cy) = cell_coordinates(&bird.position,self.cell_size);
        let neighboring_cells = self.neighboring_cells(cx,cy);
        for cell_index in neighboring_cells {
            for &candidate_index in &self.cells[cell_index] {
                if candidate_index != bird_index {
                    if bird.position.periodic_displacement(&birds[candidate_index].position,width,height).magnitude() < radius {
                        neighbors.push(candidate_index);
                    }
                }                 
            }
        }
        neighbors
    }
}

fn separation(
    birds: &[Bird],
    index: usize,
    neighbors: &[usize],
    width: f64,
    height: f64 
) -> Vec2D {
        let mut separation = Vec2D {x:0.0, y:0.0};
        let bird_i = &birds[index];
        for &neighbor in neighbors {
            let bird_j = &birds[neighbor];
            let displacement = bird_j.position.periodic_displacement(&bird_i.position,width,height);
            let distance = displacement.magnitude();
            if distance > 1e-6 {
                let contribution = displacement.scale(1.0/(distance*distance));
                separation = separation.add(&contribution);
            }
        }
        separation

}

fn cohesion(
    birds: &[Bird],
    index: usize,
    neighbors: &[usize],
    width: f64,
    height: f64
) -> Vec2D {
        let mut cohesion = Vec2D {x:0.0, y:0.0};
        let count = neighbors.len() as f64;
        let position = &birds[index].position;
        if count != 0.0 {
            for &neighbor in neighbors {
                let displacement = position.periodic_displacement(&birds[neighbor].position,width,height);
                cohesion = cohesion.add(&displacement);
            }
            cohesion = cohesion.scale(1.0/count);
        }
        cohesion    
}

fn alignment(
    birds: &[Bird],
    index: usize,
    neighbors: &[usize],
) -> Vec2D {
        let mut alignment = Vec2D {x:0.0, y:0.0};
        let count = neighbors.len() as f64;
        if count != 0.0 {
            for &neighbor in neighbors {
                alignment = alignment.add(&birds[neighbor].velocity);
            }
            alignment = alignment.scale(1.0/count);
            alignment = alignment.subtract(&birds[index].velocity);
        }
        alignment
}

fn find_neighbors_brute_force(
    birds: &[Bird],
    index: usize,
    radius: f64,
    width: f64,
    height: f64
) -> Vec<usize> {
        let mut neighbors: Vec<usize> = Vec::new();
        let bird_i = &birds[index];
        for (idx,bird_j) in birds.iter().enumerate() {
            if idx != index {
                if bird_i.position.periodic_displacement(&bird_j.position,width,height).magnitude() < radius {
                    neighbors.push(idx);
                }
            } 
        }
        neighbors        
    }

fn calculate_acceleration(
    birds: &[Bird],
    index: usize,
    params: &FlockParameters,
    grid: &SpatialGrid,
) -> Vec2D {
    let neighbors = grid.find_neighbors(birds,index,params.neighbor_radius,params.width,params.height);
    let separation = separation(birds,index,&neighbors,params.width,params.height).scale(params.separation_gain);
    let cohesion = cohesion(birds,index,&neighbors,params.width,params.height).scale(params.cohesion_gain);
    let alignment = alignment(birds,index,&neighbors).scale(params.alignment_gain);
    separation.add(&cohesion).add(&alignment)
    
}

fn create_flock(n: usize, max_speed: f64, width: f64, height: f64) -> Vec<Bird> {
    let mut rng = ::rand::rng();
    let mut birds: Vec<Bird> = Vec::with_capacity(n);
    for _ in 0..n {
        birds.push(Bird {
            position: Vec2D {x: rng.random_range(0.0..=width),y: rng.random_range(0.0..=height)},
            velocity: Vec2D {x: rng.random_range(-max_speed..=max_speed),y: rng.random_range(-max_speed..=max_speed)},
        });    
    }
    birds
}


fn draw_birds(birds: &[Bird], params: &FlockParameters) {
    let screen_x_coeff = screen_width() / (params.width as f32);
    let screen_y_coeff = screen_height() / (params.height as f32);
    for bird in birds.iter() { // draw a triangle representing each bird
        let x_draw = (bird.position.x as f32)*screen_x_coeff;
        let y_draw = (bird.position.y as f32)*screen_y_coeff;

        let front = Vec2::new(params.bird_size, 0.0);
        let back_left = Vec2::new(-params.bird_size/2.0,-params.bird_size/2.0);
        let back_right = Vec2::new(-params.bird_size/2.0,params.bird_size/2.0);
        let angle = bird.velocity.y.atan2(bird.velocity.x) as f32;
        let center = Vec2::new(x_draw,y_draw);
        let front_vertex = rotate_point(front,angle) + center;
        let back_left_vertex = rotate_point(back_left,angle) + center;
        let back_right_vertex = rotate_point(back_right,angle) + center;
        draw_triangle(front_vertex, back_left_vertex, back_right_vertex, WHITE);
    }
}

fn draw_hud(params: &FlockParameters) {
    let sep_message = format!("Q/A Separation: {:.2}", params.separation_gain);
    draw_text(&sep_message, 20.0, screen_height()-35.0, 20.0, WHITE);
    let coh_message = format!("W/S Cohesion: {:.2}", params.cohesion_gain);
    draw_text(&coh_message, 20.0, screen_height()-20.0, 20.0, WHITE);
    let ali_message = format!("E/D Alignment: {:.2}", params.alignment_gain);
    draw_text(&ali_message, 20.0, screen_height()-5.0, 20.0, WHITE);    
}

fn rotate_point(point: Vec2, angle: f32) -> Vec2 {
    Vec2 {x: point.x*(angle.cos())-point.y*(angle.sin()),
          y: point.x*(angle.sin())+point.y*(angle.cos())}
}

fn cell_coordinates(position: &Vec2D, cell_size: f64) -> (usize,usize) { // simulation canvas will be divided into a grid of cells
    let cell_x = (position.x / cell_size).floor() as usize;
    let cell_y = (position.y / cell_size).floor() as usize;
    (cell_x,cell_y)
}

fn cell_index(cx: usize, cy: usize, cols: usize) -> usize { // convert (cx,cy) cell coordinates to 1D index using row major convention
    cy*cols + cx
}

fn index_to_coordinates(index: usize, cols: usize) -> (usize,usize) { // converts 1d row major index to 2d (cx,cy) cell coordinates
    let cx = index % cols;
    let cy = index / cols;
    (cx,cy)
}

fn verify_neighbors(birds: &[Bird], grid: &SpatialGrid, radius: f64, width: f64, height: f64,) { // compares grid search find neighbors to brute force find neighbor
    for index in 0..birds.len() {
        let mut bruteforce_neighbors = find_neighbors_brute_force(birds, index, radius, width, height);
        let mut neighbors = grid.find_neighbors(birds, index, radius, width, height);
        bruteforce_neighbors.sort_unstable();
        neighbors.sort_unstable();
        assert_eq!(neighbors,bruteforce_neighbors, "mismatch for neighbors of bird {:?}",index);
    }
}

#[macroquad::main("bird flock Sim")]
async fn main() {

    let params = FlockParameters {
        separation_gain: 1.5,
        cohesion_gain: 0.05,
        alignment_gain: 0.5,
        neighbor_radius: 5.0,
        max_speed: 50.0,
        width: 100.0,
        height: 100.0,
        flock_size: 50,
        bird_size: 10.0,
    };
    let birds = create_flock(params.flock_size,params.max_speed,params.width,params.height);
    let grid = SpatialGrid::new(params.width,params.height,params.neighbor_radius);
    let mut simulation = Simulation {
        birds,
        params,
        dt: 0.01,
        accumulator: 0.0,
        accelerations: Vec::new(),
        grid: grid,
    };
    simulation.grid.rebuild(&simulation.birds);
    verify_neighbors(&simulation.birds, &simulation.grid, simulation.params.neighbor_radius, simulation.params.width, simulation.params.height);
    let sim_flocksizes = [50,100,200,400,800];
    for flocksize in sim_flocksizes {
        simulation.birds = create_flock(flocksize,simulation.params.max_speed,simulation.params.width,simulation.params.height);
        let avg_exec_time = simulation.benchmark(100);
        println!("Average step time for flock of size {:?} = {:?} ms ",flocksize,avg_exec_time);
    }

    loop {
        if is_key_pressed(KeyCode::Q) {
            simulation.params.separation_gain += 0.1;
        }
        if is_key_pressed(KeyCode::A) {
            simulation.params.separation_gain = (simulation.params.separation_gain - 0.1).max(0.0);
        }
        if is_key_pressed(KeyCode::W) {
            simulation.params.cohesion_gain += 0.01;
        }
        if is_key_pressed(KeyCode::S) {
            simulation.params.cohesion_gain = (simulation.params.cohesion_gain - 0.01).max(0.0);
        }
        if is_key_pressed(KeyCode::E) {
            simulation.params.alignment_gain += 0.1;
        }
        if is_key_pressed(KeyCode::D) {
            simulation.params.alignment_gain = (simulation.params.alignment_gain - 0.1).max(0.0);
        }
        simulation.accumulator += (get_frame_time() as f64).min(0.1);
        while simulation.accumulator >= simulation.dt {
            simulation.step();
            simulation.accumulator -= simulation.dt;
        }    
        clear_background(BLACK);
        draw_birds(&simulation.birds,&simulation.params);
        draw_hud(&simulation.params);
        next_frame().await;
    }
}