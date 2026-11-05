use crate::bird::Bird;
use crate::vector2d::Vec2D;

pub(crate) struct SpatialGrid { // divide simulation into a grid of square cells. cells is a list of lists, where the inner list is the birds occupying each cell.
    cells: Vec<Vec<usize>>,
    cols: usize, // number of cols in grid
    rows: usize, // number of rows in grid
    cell_size: f64, // width and height of each cell
}

impl SpatialGrid {
    pub(crate) fn new(sim_width: f64, sim_height: f64, cell_size: f64) -> Self {
        let cols = (sim_width / cell_size).ceil() as usize;
        let rows = (sim_height / cell_size).ceil() as usize;
        let cells = vec![Vec::<usize>::new(); cols*rows];
        SpatialGrid {
            cells,
            cols,
            rows,
            cell_size,
        }
    }
    pub(crate) fn rebuild(&mut self, birds: &[Bird]) { // update the neighbor information in each cell after birds move
        for cell in self.cells.iter_mut() {
            cell.clear();
        }
        for (bird_index,bird) in birds.iter().enumerate() {
            let (cx,cy) = cell_coordinates(&bird.position(), self.cell_size);
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
                neighbor_cells.push(cell_index(wrapped_x,wrapped_y,self.cols));
            }
        }
        neighbor_cells
    }
    pub(crate) fn find_neighbors(&self, birds: &[Bird], bird_index: usize, neighbor_radius: f64, sim_width: f64, sim_height: f64,) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let bird = &birds[bird_index];
        let (cx,cy) = cell_coordinates(&bird.position(),self.cell_size);
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

