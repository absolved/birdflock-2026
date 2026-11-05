fn verify_neighbors(birds: &[Bird], grid: &SpatialGrid, radius: f64, width: f64, height: f64,) { // compares grid search find neighbors to brute force find neighbor
    for index in 0..birds.len() {
        let mut bruteforce_neighbors = find_neighbors_brute_force(birds, index, radius, width, height);
        let mut neighbors = grid.find_neighbors(birds, index, radius, width, height);
        bruteforce_neighbors.sort_unstable();
        neighbors.sort_unstable();
        assert_eq!(neighbors,bruteforce_neighbors, "mismatch for neighbors of bird {:?}",index);
    }
}

fn find_neighbors_brute_force(
    birds: &[Bird],
    index: usize,
    neighbor_radius: f64,
    sim_width: f64,
    sim_height: f64
) -> Vec<usize> {
        let mut neighbors: Vec<usize> = Vec::new();
        let bird_i = &birds[index];
        for (idx,bird_j) in birds.iter().enumerate() {
            if idx != index {
                if bird_i.position.periodic_displacement(&bird_j.position,sim_width,sim_height).magnitude() < neighbor_radius {
                    neighbors.push(idx);
                }
            } 
        }
        neighbors        
    }