use crate::vector2d::Vec2D;
use crate::bird::Bird;
use crate::spatial_grid::SpatialGrid;

pub struct FlockParameters {
    pub separation_gain: f64,
    pub cohesion_gain: f64,
    pub alignment_gain: f64,
    pub neighbor_radius: f64,
    pub max_speed: f64,
    pub sim_width: f64,
    pub sim_height: f64,
    pub cell_size: f64,
    pub bird_size: f32,
}

fn separation(
    birds: &[Bird],
    index: usize,
    neighbors: &[usize],
    sim_width: f64,
    sim_height: f64 
) -> Vec2D {
        let mut separation = Vec2D {x:0.0, y:0.0};
        let bird_i = &birds[index];
        for &neighbor in neighbors {
            let bird_j = &birds[neighbor];
            let displacement = bird_j.position().periodic_displacement(&bird_i.position(),sim_width,sim_height);
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
) -> Vec2D {
        let mut alignment = Vec2D {x:0.0, y:0.0};
        let count = neighbors.len() as f64;
        if count != 0.0 {
            for &neighbor in neighbors {
                alignment = alignment.add(&birds[neighbor].velocity());
            }
            alignment = alignment.scale(1.0/count);
            alignment = alignment.subtract(&birds[index].velocity());
        }
        alignment
}

pub(crate) fn calculate_acceleration(
    birds: &[Bird],
    index: usize,
    params: &FlockParameters,
    grid: &SpatialGrid,
) -> Vec2D {
    let neighbors = grid.find_neighbors(birds,index,params.neighbor_radius,params.sim_width,params.sim_height);
    let separation = separation(birds,index,&neighbors,params.sim_width,params.sim_height).scale(params.separation_gain);
    let cohesion = cohesion(birds,index,&neighbors,params.sim_width,params.sim_height).scale(params.cohesion_gain);
    let alignment = alignment(birds,index,&neighbors).scale(params.alignment_gain);
    separation.add(&cohesion).add(&alignment)    
}