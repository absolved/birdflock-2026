use crate::vector2d::Vec2D;
use crate::bird::Bird;
use crate::spatial_grid::SpatialGrid;

pub struct FlockParameters {
    pub separation_gain: f64,
    pub cohesion_gain: f64,
    pub alignment_gain: f64,
    pub global_cohesion_gain: f64,
    pub neighbor_radius: f64,
    pub separation_radius: f64,
    pub collision_radius: f64,
    pub max_speed: f64,
    pub alignment_speed_gain: f64, // ratio to max speed in (0,1]
    pub sim_width: f64,
    pub sim_height: f64,
    pub cell_size: f64,
    pub bird_size: f32,
    pub destination_interval: f64,
    pub separation_softening: f64,
}

fn separation(
    birds: &[Bird],
    index: usize,
    neighbors: &[usize],
    params: &FlockParameters,
) -> Vec2D {
        const MIN_DISTANCE: f64 = 1e-6;
        let mut separation = Vec2D {x:0.0, y:0.0};
        let bird_i = &birds[index];
        for &neighbor in neighbors {
            let bird_j = &birds[neighbor];
            let displacement = bird_j.position().periodic_displacement(&bird_i.position(),params.sim_width,params.sim_height);
            let distance = displacement.magnitude();
            if distance > MIN_DISTANCE && distance < params.separation_radius {
                let separation_ratio = ((params.separation_radius-distance)/(distance+params.separation_softening)).powi(2);
                let contribution = displacement.scale(1.0/distance).scale(separation_ratio);
                separation = separation.add(&contribution);
            }
        }
        separation
}

fn cohesion(
    birds: &[Bird],
    index: usize,
    neighbors: &[usize],
    sim_width: f64,
    sim_height: f64
) -> Vec2D {
        let mut cohesion = Vec2D {x:0.0, y:0.0};
        let count = neighbors.len() as f64;
        let position = &birds[index].position();
        if count != 0.0 {
            for &neighbor in neighbors {
                let displacement = position.periodic_displacement(&birds[neighbor].position(),sim_width,sim_height);
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
    params: &FlockParameters,
) -> Vec2D {
        let mut alignment = Vec2D {x:0.0, y:0.0};
        let count = neighbors.len() as f64;
        if count != 0.0 {
            let alignment_speed = params.alignment_speed_gain * params.max_speed;
            for &neighbor in neighbors {
                alignment = alignment.add(&birds[neighbor].velocity());
            }
            alignment = alignment.scale(1.0/count);
            let alignment_magnitude = alignment.magnitude();
            if alignment_magnitude > 1e-12 {
                alignment = alignment.scale(alignment_speed / alignment_magnitude);
                alignment = alignment.subtract(&birds[index].velocity())
            }            
        }
          
        alignment
}

pub(crate) fn calculate_acceleration(
    birds: &[Bird],
    index: usize,
    params: &FlockParameters,
    grid: &SpatialGrid,
    flock_destination: &Vec2D,
) -> Vec2D {
    let neighbors = grid.find_neighbors(birds,index,params.neighbor_radius,params.sim_width,params.sim_height);
    let separation = separation(birds,index,&neighbors,&params).scale(params.separation_gain);
    let cohesion = cohesion(birds,index,&neighbors,params.sim_width,params.sim_height).scale(params.cohesion_gain);
    let alignment = alignment(birds,index,&neighbors,params).scale(params.alignment_gain);
    let global_cohesion = birds[index].position().periodic_displacement(flock_destination, params.sim_width, params.sim_height).scale(params.global_cohesion_gain);
    separation.add(&cohesion).add(&alignment).add(&global_cohesion)    
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
    fn test_cohesion_basic() {
        let sim_width = 100.0;
        let sim_height = 100.0;
        let velocity = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 10.0 }, velocity),
            Bird::new(Vec2D { x: 20.0, y: 10.0 }, velocity),
            Bird::new(Vec2D { x: 30.0, y: 10.0 }, velocity),
        ];

        let neighbors = vec![1, 2];

        let cohesion = cohesion(&birds, 0, &neighbors, sim_width, sim_height);
        assert!((cohesion.x-15.0).abs() < EPSILON, "Expected cohesion.x = 15.0 got {}", cohesion.x);
        assert!((cohesion.y-0.0).abs() < EPSILON, "Expected cohesion.y = 0.0 got {}", cohesion.y);    
    }

    #[test]
    fn test_cohesion_periodic() {
        let sim_width = 100.0;
        let sim_height = 100.0;
        let velocity = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(Vec2D { x: 4.0, y: 50.0 }, velocity),
            Bird::new(Vec2D { x: 2.0, y: 50.0 }, velocity),
            Bird::new(Vec2D { x: 98.0, y: 50.0 }, velocity),
        ];

        let neighbors = vec![1, 2];

        let cohesion = cohesion(&birds, 0, &neighbors, sim_width, sim_height);
        assert!((cohesion.x+4.0).abs() < EPSILON, "Expected cohesion.x = -4.0 got {}", cohesion.x);
        assert!((cohesion.y-0.0).abs() < EPSILON, "Expected cohesion.y = 0.0 got {}", cohesion.y);
    }

    #[test]
    fn test_cohesion_no_neighbors() {
        let sim_width = 100.0;
        let sim_height = 100.0;
        let velocity = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(Vec2D { x: 50.0, y: 50.0 }, velocity),
        ];

        let neighbors = vec![];

        let cohesion = cohesion(&birds, 0, &neighbors, sim_width, sim_height);
        assert!((cohesion.x-0.0).abs() < EPSILON, "Expected cohesion.x = 0.0 got {}", cohesion.x);
        assert!((cohesion.y-0.0).abs() < EPSILON, "Expected cohesion.y = 0.0 got {}", cohesion.y);
    }

    #[test]
    fn test_alignment_basic() {
        let params = test_params();
        let position = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(position, Vec2D { x: 0.0, y: 2.0 }),
            Bird::new(position, Vec2D { x: 3.0, y: 0.0 }),
            Bird::new(position, Vec2D { x: 5.0, y: 0.0 }),
        ];

        let neighbors = vec![1, 2];

        let alignment = alignment(&birds, 0, &neighbors, &params);
        assert!((alignment.x-10.0).abs() < EPSILON, "Expected alignment.x = 10.0 got {}", alignment.x);
        assert!((alignment.y+2.0).abs() < EPSILON, "Expected alignment.y = -2.0 got {}", alignment.y);
    }

    #[test]
    fn test_alignment_opposing_velocities() {
        let params = test_params();

        let position = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(position, Vec2D { x: 3.0, y: 4.0 }),
            Bird::new(position, Vec2D { x: 5.0, y: 0.0 }),
            Bird::new(position, Vec2D { x: -5.0, y: 0.0 }),
        ];

        let neighbors = vec![1, 2];

        let alignment = alignment(&birds, 0, &neighbors, &params);
        assert!((alignment.x-0.0).abs() < EPSILON, "Expected alignment.x = 0.0 got {}", alignment.x);
        assert!((alignment.y-0.0).abs() < EPSILON, "Expected alignment.y = 0.0 got {}", alignment.y);
    }

    #[test]
    fn test_alignment_no_neighbors() {
        let params = test_params();

        let position = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(position, Vec2D { x: 3.0, y: 4.0 }),
        ];

        let neighbors = vec![];

        let alignment = alignment(&birds, 0, &neighbors, &params);
        assert!((alignment.x-0.0).abs() < EPSILON, "Expected alignment.x = 0.0 got {}", alignment.x);
        assert!((alignment.y-0.0).abs() < EPSILON, "Expected alignment.y = 0.0 got {}", alignment.y);
    }

    #[test]
    fn test_separation_basic() {
        let params = test_params();

        let velocity = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 10.0 }, velocity),
            Bird::new(Vec2D { x: 14.0, y: 10.0 }, velocity),
        ];

        let neighbors = vec![1];

        let separation = separation(&birds, 0, &neighbors, &params);
        let expected_x = -(4.0_f64 / 4.1).powi(2);
        assert!((separation.x-expected_x).abs() < EPSILON, "Expected separation.x = {} got {}", expected_x, separation.x);
        assert!((separation.y-0.0).abs() < EPSILON, "Expected separation.y = 0.0 got {}", separation.y);    
    }

    #[test]
    fn test_separation_periodic() {
        let params = test_params();

        let velocity = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(Vec2D { x: 2.0, y: 50.0 }, velocity),
            Bird::new(Vec2D { x: 98.0, y: 50.0 }, velocity),
        ];

        let neighbors = vec![1];

        let separation = separation(&birds, 0, &neighbors, &params);
        let expected_x = (4.0_f64 / 4.1).powi(2);
        assert!((separation.x-expected_x).abs() < EPSILON, "Expected separation.x = {} got {}", expected_x, separation.x);
        assert!((separation.y-0.0).abs() < EPSILON, "Expected separation.y = 0.0 got {}", separation.y);    
    }

    #[test]
    fn test_separation_zero_distance() {
        let params = test_params();

        let velocity = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(Vec2D { x: 50.0, y: 50.0 }, velocity),
            Bird::new(Vec2D { x: 50.0, y: 50.0 }, velocity),
        ];

        let neighbors = vec![1];

        let separation = separation(&birds, 0, &neighbors, &params);
        assert!((separation.x-0.0).abs() < EPSILON, "Expected separation.x = 0.0 got {}", separation.x);
        assert!((separation.y-0.0).abs() < EPSILON, "Expected separation.y = 0.0 got {}", separation.y);    
    }

    #[test]
    fn test_separation_radius_cutoff() {
        let params = test_params();

        let velocity = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 10.0 }, velocity),
            Bird::new(Vec2D { x: 18.0, y: 10.0 }, velocity),
            Bird::new(Vec2D { x: 19.0, y: 10.0 }, velocity),
        ];

        let neighbors = vec![1,2];

        let separation = separation(&birds, 0, &neighbors, &params);
        assert!((separation.x-0.0).abs() < EPSILON, "Expected separation.x = 0.0 got {}", separation.x);
        assert!((separation.y-0.0).abs() < EPSILON, "Expected separation.y = 0.0 got {}", separation.y);    
    }

    #[test]
    fn test_separation_multiple_neighbors() {
        let params = test_params();

        let velocity = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(Vec2D { x: 50.0, y: 50.0 }, velocity),
            Bird::new(Vec2D { x: 54.0, y: 50.0 }, velocity),
            Bird::new(Vec2D { x: 50.0, y: 53.0 }, velocity),
        ];

        let neighbors = vec![1,2];

        let separation = separation(&birds, 0, &neighbors, &params);
        let expected_x = -(4.0_f64 / 4.1).powi(2);
        let expected_y = -(5.0_f64 / 3.1).powi(2);
        assert!((separation.x-expected_x).abs() < EPSILON, "Expected separation.x = {} got {}", expected_x, separation.x);
        assert!((separation.y-expected_y).abs() < EPSILON, "Expected separation.y = {} got {}", expected_y, separation.y);    
    }

    #[test]
    fn test_separation_softening() {
        let params_a = test_params();
        let params_b = FlockParameters {
            separation_softening: 0.5,
            ..test_params()
        };

        let velocity = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 10.0 }, velocity),
            Bird::new(Vec2D { x: 14.0, y: 10.0 }, velocity),
        ];

        let neighbors = vec![1];

        let separation_a = separation(&birds, 0, &neighbors, &params_a);
        let separation_b = separation(&birds, 0, &neighbors, &params_b);
        let expected_x_a = -(4.0_f64 / 4.1).powi(2);
        let expected_x_b = -(4.0_f64 / 4.5).powi(2);
        assert!((separation_a.x-expected_x_a).abs() < EPSILON, "Expected separation_a.x = {} got {}", expected_x_a, separation_a.x);
        assert!((separation_a.y-0.0).abs() < EPSILON, "Expected separation_a.y = 0.0 got {}", separation_a.y);
        assert!((separation_b.x-expected_x_b).abs() < EPSILON, "Expected separation_b.x = {} got {}", expected_x_b, separation_b.x);
        assert!((separation_b.y-0.0).abs() < EPSILON, "Expected separation_b.y = 0.0 got {}", separation_b.y);
        assert!(separation_a.x.abs() > separation_b.x.abs());  
    }

    #[test]
    fn test_calculate_acceleration_separation_only() {
        let params = FlockParameters {
            separation_gain: 2.0,
            cohesion_gain: 0.0,
            alignment_gain: 0.0,
            global_cohesion_gain: 0.0,
            ..test_params()
        }; 

        let velocity = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 10.0 }, velocity),
            Bird::new(Vec2D { x: 14.0, y: 10.0 }, velocity),
        ];

        let mut grid = SpatialGrid::new(params.sim_width, params.sim_height, params.cell_size);
        grid.rebuild(&birds);

        let destination = Vec2D {x: 50.0, y: 50.0};
        let acceleration = calculate_acceleration(&birds, 0, &params, &grid, &destination);

        let expected_x = -2.0 * (4.0_f64 / 4.1).powi(2);
        let expected_y = 0.0;
        assert!((acceleration.x-expected_x).abs() < EPSILON, "Expected acceleration.x = {} got {}", expected_x, acceleration.x);
        assert!((acceleration.y-expected_y).abs() < EPSILON, "Expected acceleration.y = {} got {}", expected_y, acceleration.y); 
    }

    #[test]
    fn test_calculate_acceleration_cohesion_only() {
        let params = FlockParameters {
            separation_gain: 0.0,
            cohesion_gain: 2.0,
            alignment_gain: 0.0,
            global_cohesion_gain: 0.0,
            ..test_params()
        }; 

        let velocity = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 10.0 }, velocity),
            Bird::new(Vec2D { x: 14.0, y: 10.0 }, velocity),
            Bird::new(Vec2D { x: 10.0, y: 16.0 }, velocity),
        ];

        let mut grid = SpatialGrid::new(params.sim_width, params.sim_height, params.cell_size);
        grid.rebuild(&birds);

        let destination = Vec2D {x: 50.0, y: 50.0};
        let acceleration = calculate_acceleration(&birds, 0, &params, &grid, &destination);

        let expected_x = 4.0;
        let expected_y = 6.0;
        assert!((acceleration.x-expected_x).abs() < EPSILON, "Expected acceleration.x = {} got {}", expected_x, acceleration.x);
        assert!((acceleration.y-expected_y).abs() < EPSILON, "Expected acceleration.y = {} got {}", expected_y, acceleration.y); 
    }

    #[test]
    fn test_calculate_acceleration_alignment_only() {
        let params = FlockParameters {
            separation_gain: 0.0,
            cohesion_gain: 0.0,
            alignment_gain: 2.0,
            global_cohesion_gain: 0.0,
            max_speed: 10.0,
            alignment_speed_gain: 1.0,
            ..test_params()
        }; 

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 10.0 }, Vec2D { x: 1.0, y: 2.0}),
            Bird::new(Vec2D { x: 14.0, y: 10.0 }, Vec2D { x: 3.0, y: 0.0}),
            Bird::new(Vec2D { x: 10.0, y: 16.0 }, Vec2D { x: 0.0, y: 4.0}),
        ];

        let mut grid = SpatialGrid::new(params.sim_width, params.sim_height, params.cell_size);
        grid.rebuild(&birds);

        let destination = Vec2D {x: 50.0, y: 50.0};
        let acceleration = calculate_acceleration(&birds, 0, &params, &grid, &destination);

        let expected_x = 10.0;
        let expected_y = 12.0;
        assert!((acceleration.x-expected_x).abs() < EPSILON, "Expected acceleration.x = {} got {}", expected_x, acceleration.x);
        assert!((acceleration.y-expected_y).abs() < EPSILON, "Expected acceleration.y = {} got {}", expected_y, acceleration.y); 
    }

    #[test]
    fn test_calculate_acceleration_global_cohesion_only() {
        let params = FlockParameters {
            separation_gain: 0.0,
            cohesion_gain: 0.0,
            alignment_gain: 0.0,
            global_cohesion_gain: 0.5,
            ..test_params()
        }; 

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 20.0 }, Vec2D { x: 0.0, y: 0.0}),
        ];

        let mut grid = SpatialGrid::new(params.sim_width, params.sim_height, params.cell_size);
        grid.rebuild(&birds);

        let destination = Vec2D {x: 40.0, y: 60.0};
        let acceleration = calculate_acceleration(&birds, 0, &params, &grid, &destination);

        let expected_x = 15.0;
        let expected_y = 20.0;
        assert!((acceleration.x-expected_x).abs() < EPSILON, "Expected acceleration.x = {} got {}", expected_x, acceleration.x);
        assert!((acceleration.y-expected_y).abs() < EPSILON, "Expected acceleration.y = {} got {}", expected_y, acceleration.y); 
    }


    #[test]
    fn test_calculate_acceleration_global_cohesion_periodic() {
        let params = FlockParameters {
            separation_gain: 0.0,
            cohesion_gain: 0.0,
            alignment_gain: 0.0,
            global_cohesion_gain: 0.5,
            ..test_params()
        }; 

        let birds = vec![
            Bird::new(Vec2D { x: 98.0, y: 50.0 }, Vec2D { x: 0.0, y: 0.0}),
        ];

        let mut grid = SpatialGrid::new(params.sim_width, params.sim_height, params.cell_size);
        grid.rebuild(&birds);

        let destination = Vec2D {x: 2.0, y: 50.0};
        let acceleration = calculate_acceleration(&birds, 0, &params, &grid, &destination);

        let expected_x = 2.0;
        let expected_y = 0.0;
        assert!((acceleration.x-expected_x).abs() < EPSILON, "Expected acceleration.x = {} got {}", expected_x, acceleration.x);
        assert!((acceleration.y-expected_y).abs() < EPSILON, "Expected acceleration.y = {} got {}", expected_y, acceleration.y); 
    }        

    #[test]
    fn test_calculate_acceleration_all_components() {
        let params = FlockParameters {
            separation_gain: 2.0,
            cohesion_gain: 0.5,
            alignment_gain: 1.5,
            global_cohesion_gain: 0.1,
            max_speed: 10.0,
            alignment_speed_gain: 1.0,
            ..test_params()
        };

        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 10.0 }, Vec2D { x: 1.0, y: 2.0}),
            Bird::new(Vec2D { x: 14.0, y: 10.0 }, Vec2D { x: 3.0, y: 0.0}),
            Bird::new(Vec2D { x: 10.0, y: 16.0 }, Vec2D { x: 0.0, y: 4.0}),
        ];

        let mut grid = SpatialGrid::new(params.sim_width, params.sim_height, params.cell_size);
        grid.rebuild(&birds);

        let destination = Vec2D {x: 50.0, y: 30.0};
        let acceleration = calculate_acceleration(&birds, 0, &params, &grid, &destination);

        let expected_x = -2.0 * (4.0_f64 / 4.1).powi(2) + 1.0 + 7.5 + 4.0;
        let expected_y = -2.0 * (2.0_f64 / 6.1).powi(2) + 1.5 + 9.0 + 2.0;
        assert!((acceleration.x-expected_x).abs() < EPSILON, "Expected acceleration.x = {} got {}", expected_x, acceleration.x);
        assert!((acceleration.y-expected_y).abs() < EPSILON, "Expected acceleration.y = {} got {}", expected_y, acceleration.y);        
    }
}