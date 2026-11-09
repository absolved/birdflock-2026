use crate::bird::Bird;
use crate::vector2d::Vec2D;

pub(crate) struct SpatialGrid { // divide simulation into a grid of square cells. cells is a list of lists, where the inner list is the birds occupying each cell.
    cells: Vec<Vec<usize>>,
    cols: usize, // number of cols in grid
    rows: usize, // number of rows in grid
    cell_width: f64, // width of each cell
    cell_height: f64, // height of each cell
}

impl SpatialGrid {
    pub(crate) fn new(sim_width: f64, sim_height: f64, cell_size: f64,) -> Self {
        let cols = ((sim_width / cell_size).floor() as usize).max(1);
        let rows = ((sim_height / cell_size).floor() as usize).max(1);
        let cells = vec![Vec::<usize>::new(); cols*rows];
        let cell_width = sim_width / cols as f64;
        let cell_height = sim_height / rows as f64;
        SpatialGrid {
            cells,
            cols,
            rows,
            cell_width,
            cell_height,
        }
    }
    pub(crate) fn rebuild(&mut self, birds: &[Bird]) { // update the neighbor information in each cell after birds move
        for cell in self.cells.iter_mut() {
            cell.clear();
        }
        for (bird_index,bird) in birds.iter().enumerate() {
            let (cx,cy) = cell_coordinates(&bird.position(), self.cell_width, self.cell_height);
            let index = cell_index(cx,cy, self.cols);
            self.cells[index].push(bird_index);
        }
    }
    pub(crate) fn neighboring_cells(&self, cx: usize, cy: usize) -> Vec<usize> {
        let mut neighbor_cells: Vec<usize> = Vec::new(); 
        for dy in -1..=1 {
            for dx in -1..=1 {
                let wrapped_x = (cx as isize + dx).rem_euclid(self.cols as isize) as usize;
                let wrapped_y = (cy as isize + dy).rem_euclid(self.rows as isize) as usize;
                let index = cell_index(wrapped_x,wrapped_y,self.cols);

                if !neighbor_cells.contains(&index) {
                    neighbor_cells.push(index);
                }
                
            }
        }
        neighbor_cells
    }
    pub(crate) fn find_neighbors(&self, birds: &[Bird], bird_index: usize, neighbor_radius: f64, sim_width: f64, sim_height: f64,) -> Vec<usize> {
        assert!(neighbor_radius <= self.cell_width && neighbor_radius <= self.cell_height, "Neighbor radius exceeds spatial grid cell dimensions");
        let mut neighbors = Vec::new();
        let bird = &birds[bird_index];
        let (cx,cy) = cell_coordinates(&bird.position(),self.cell_width,self.cell_height);
        let neighboring_cells = self.neighboring_cells(cx,cy);
        for cell_index in neighboring_cells {
            for &candidate_index in &self.cells[cell_index] {
                if candidate_index != bird_index {
                    if bird.position().periodic_displacement(&birds[candidate_index].position(),sim_width,sim_height).magnitude() < neighbor_radius {
                        neighbors.push(candidate_index);
                    }
                }                 
            }
        }
        neighbors
    }
}

fn cell_coordinates(position: &Vec2D, cell_width: f64, cell_height: f64) -> (usize,usize) { // simulation canvas will be divided into a grid of cells
    let cell_x = (position.x / cell_width).floor() as usize;
    let cell_y = (position.y / cell_height).floor() as usize;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bird::Bird;

    fn verify_neighbors(birds: &[Bird], grid: &SpatialGrid, radius: f64, width: f64, height: f64,) { // compares grid search find neighbors to brute force find neighbor
        for index in 0..birds.len() {
            let mut bruteforce_neighbors = find_neighbors_brute_force(birds, index, radius, width, height);
            let mut neighbors = grid.find_neighbors(birds, index, radius, width, height);
            bruteforce_neighbors.sort_unstable();
            neighbors.sort_unstable();
            assert_eq!(neighbors,bruteforce_neighbors, "mismatch for neighbors of bird {:?}",index);
        }
    }

    fn find_neighbors_brute_force(birds: &[Bird], index: usize, neighbor_radius: f64, sim_width: f64, sim_height: f64) -> Vec<usize> {
        let mut neighbors: Vec<usize> = Vec::new();
        let bird_i = &birds[index];
        for (idx,bird_j) in birds.iter().enumerate() {
            if idx != index {
                let displacement = bird_i.position().periodic_displacement(&bird_j.position(),sim_width,sim_height);
                if displacement.magnitude() < neighbor_radius {
                    neighbors.push(idx);
                }
            } 
        }
        neighbors        
    }

    #[test]
    fn test_neighbor_search_basic() {
        let sim_width = 100.0;
        let sim_height = 100.0;
        let neighbor_radius = 5.0;
        let cell_size = 5.0;
        let velocity = Vec2D {x:0.0, y:0.0};
        let birds = vec![
            Bird::new(Vec2D { x: 10.0, y: 10.0 }, velocity),
            Bird::new(Vec2D { x: 12.0, y: 10.0 }, velocity),
            Bird::new(Vec2D { x: 30.0, y: 30.0 }, velocity),
            Bird::new(Vec2D { x: 80.0, y: 80.0 }, velocity),
            ];

        let mut grid = SpatialGrid::new(sim_width, sim_height, cell_size);
        grid.rebuild(&birds);
        let neighbors = grid.find_neighbors(&birds, 0, neighbor_radius, sim_width, sim_height);
        assert_eq!(neighbors, vec![1]);

        verify_neighbors(&birds, &grid, neighbor_radius, sim_width, sim_height,);        
    }

    #[test]
    fn test_neighbor_search_periodic() {
        let sim_width = 100.0;
        let sim_height = 100.0;
        let neighbor_radius = 5.0;
        let cell_size = 5.0;
        let velocity = Vec2D {x:0.0, y:0.0};
        let birds = vec![
            Bird::new(Vec2D { x: 2.0, y: 50.0 }, velocity),
            Bird::new(Vec2D { x: 98.0, y: 50.0 }, velocity),
            Bird::new(Vec2D { x: 50.0, y: 2.0 }, velocity),
            Bird::new(Vec2D { x: 50.0, y: 98.0 }, velocity),
            ];

        let mut grid = SpatialGrid::new(sim_width, sim_height, cell_size);
        grid.rebuild(&birds);
        let neighbors = grid.find_neighbors(&birds, 0, neighbor_radius, sim_width, sim_height);
        assert_eq!(neighbors, vec![1]);
        let neighbors = grid.find_neighbors(&birds, 2, neighbor_radius, sim_width, sim_height);
        assert_eq!(neighbors, vec![3]);

        verify_neighbors(&birds, &grid, neighbor_radius, sim_width, sim_height,);        
    }

    #[test]
    fn test_partial_cell_boundary() {
        let sim_width = 100.0;
        let sim_height = 100.0;
        let cell_size = 6.0;
        let neighbor_radius = 5.0;

        let velocity = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(Vec2D { x: 0.5, y: 50.0 }, velocity),
            Bird::new(Vec2D { x: 95.9, y: 50.0 }, velocity),
        ];

        let mut grid = SpatialGrid::new(sim_width, sim_height, cell_size);
        grid.rebuild(&birds);
        
        verify_neighbors(&birds, &grid, neighbor_radius, sim_width, sim_height,);    
    }

    #[test]
    fn test_small_grid_duplicate_neighbors() {
        let sim_width = 10.0;
        let sim_height = 10.0;
        let cell_size = 5.0;
        let neighbor_radius = 5.0;

        let velocity = Vec2D { x: 0.0, y: 0.0 };

        let birds = vec![
            Bird::new(Vec2D { x: 2.0, y: 2.0 }, velocity),
            Bird::new(Vec2D { x: 6.9, y: 2.0 }, velocity),
        ];

        let mut grid = SpatialGrid::new(sim_width, sim_height, cell_size);
        grid.rebuild(&birds);

        let neighbors = grid.find_neighbors(&birds, 0, neighbor_radius, sim_width, sim_height,);

        assert_eq!(neighbors, vec![1]);

        verify_neighbors(&birds, &grid, neighbor_radius, sim_width, sim_height,);    
    }

    #[test]
    fn test_randomzied_neighbor_search() {
        use ::rand::rngs::StdRng;
        use rand::{RngExt, SeedableRng};

        let mut rng = StdRng::seed_from_u64(42);

        let sim_width = 100.0;
        let sim_height = 100.0;
        let neighbor_radius = 5.0;
        let cell_size = 6.0;
        let flock_size = 200;
        let velocity = Vec2D {x:0.0, y:0.0};
        let birds: Vec<Bird> = (0..flock_size)
            .map(|_| {
                let position = Vec2D {x:rng.random_range(0.0..sim_width),y: rng.random_range(0.0..sim_height),};
                Bird::new(position,velocity)
            }).collect();

        let mut grid = SpatialGrid::new(sim_width, sim_height, cell_size);
        grid.rebuild(&birds);
        verify_neighbors(&birds, &grid, neighbor_radius, sim_width, sim_height,);

    }
    #[test]
    #[should_panic(expected = "Neighbor radius exceeds spatial grid cell dimensions")]
    fn test_neighbor_radius_exceeds_cell_size() {
        let sim_width = 100.0;
        let sim_height = 100.0;
        let cell_size = 5.0;
        let neighbor_radius = 6.0;

        let birds = vec![
            Bird::new(
                Vec2D { x: 4.9, y: 10.0 },
                Vec2D { x: 0.0, y: 0.0 },
            ),
            Bird::new(
                Vec2D { x: 10.1, y: 10.0 },
                Vec2D { x: 0.0, y: 0.0 },
            ),
        ];

        let mut grid = SpatialGrid::new(sim_width, sim_height, cell_size);
        grid.rebuild(&birds);

        grid.find_neighbors(&birds, 0, neighbor_radius, sim_width, sim_height,);        
    }
}
