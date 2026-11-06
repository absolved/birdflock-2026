# Birds — 2D Flocking Simulation

An interactive, real-time 2D flocking simulation written in Rust using Macroquad.

The project implements the Boids model of collective motion, in which simple local interaction rules produce emergent flocking behavior. It also serves as a practical exploration of Rust software engineering, numerical simulation, spatial data structures, and performance optimization.

## Features

- **Boids flocking behavior**: Separation, alignment, and cohesion rules with independently adjustable gains.
- **Real-time visualization**: Birds are rendered as directional triangles using Macroquad.
- **Interactive controls**: Adjust flocking behavior during simulation.
- **Spatial partitioning**: Uniform spatial grid accelerates neighbor searches.
- **Periodic boundaries**: Birds wrap around the simulation domain, with neighbor distances calculated accordingly.
- **Fixed-timestep integration**: Simulation updates are decoupled from rendering frequency.
- **Performance benchmarking**: Measure average simulation step execution time.
- **Modular architecture**: Simulation logic is separated from rendering and application-level input handling.

## Getting Started

### Prerequisites

- Rust and Cargo (install using rustup)

- A desktop environment capable of running Macroquad applications

### Build and Run

Clone the repository and navigate to the project directory.

```bash
cargo build
cargo run --release
```

Release mode is recommended for performance testing and simulations with larger bird populations.

### Testing (NOT YET IMPLEMENTED)

Run the test suite:

```bash
cargo test
```

Check formatting and run the Rust linter:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets
```

## Interactive Controls

|Key | Action |
 |---|---|
| Q / A | Increase / decrease separation gain |
| W / S | Increase / decrease cohesion gain |
| E / D | Increase / decrease alignment gain |

Parameter changes take effect during simulation.

## Project Structure

```bash
src/
├── main.rs          # Application entry point and input handling
├── lib.rs           # Library module declarations
├── vector2d.rs      # 2D vector operations
├── bird.rs          # Bird state and dynamics
├── flock.rs         # Flocking rules and acceleration calculations
├── spatial_grid.rs  # Spatial partitioning and neighbor search
├── simulation.rs    # Simulation state, integration, and benchmarking
└── rendering.rs     # Macroquad visualization

tests/
└── neighbor_search.rs
```

The application is organized into a reusable simulation library and a visualization executable.

The Simulation struct manages bird state, integration, and spatial indexing. The executable handles rendering, keyboard input, and frame timing.

## Simulation Model

## Flocking Dynamics

Each bird's acceleration is calculated from four behavioral components:

```math
\mathbf{a}_i =
\mathbf{a}_{s,i}
+ \mathbf{a}_{c,i}
+ \mathbf{a}_{a,i}
+ \mathbf{a}_{g,i}
```

### Separation

Nonlinear repulsion from nearby birds:

```math
\mathbf{a}_{s,i}
=
k_s \sum_{\substack{j\in\mathcal{N}_i\\0<d_{ij}<r_s}}
\frac{\mathbf{d}_{ji}}{d_{ij}}
\left(
\frac{r_s-d_{ij}}{d_{ij}+\varepsilon}
\right)^2
```

### Cohesion

Attraction toward the average displacement of neighboring birds:

```math
\mathbf{a}_{c,i}
=
\frac{k_c}{|\mathcal{N}_i|}
\sum_{j\in\mathcal{N}_i}\mathbf{d}_{ij}
```

### Alignment

Steering toward the average neighbor velocity, normalized
to the maximum desired speed:

```math
\mathbf{a}_{a,i}
=
k_a\left(
v_{\max}
\frac{\bar{\mathbf{v}}_i}{\|\bar{\mathbf{v}}_i\|}
-\mathbf{v}_i
\right)
```

where:

```math
\bar{\mathbf{v}}_i =
\frac{1}{|\mathcal{N}_i|}
\sum_{j\in\mathcal{N}_i}\mathbf{v}_j
```

### Global Cohesion

Attraction toward a shared, periodically changing random destination:

```math
\mathbf{a}_{g,i}
=
k_g(\mathbf{p}_g-\mathbf{p}_i)
```
### Notation

- `i`: Current bird
- `j`: Neighboring bird
- `N_i`: Set of neighbors of bird `i`
- `p_i`: Position of bird `i`
- `v_i`: Velocity of bird `i`
- `p_g`: Shared global destination
- `d_ij`: Shortest periodic displacement from bird `i` to bird `j`
- `d_ji`: Shortest periodic displacement from bird `j` to bird `i`
- `d_ij` (scalar): Distance between birds `i` and `j`
- `r_s`: Separation radius
- `v_max`: Maximum bird speed
- `epsilon`: Small regularization constant
- `k_s`, `k_c`, `k_a`, `k_g`: Behavioral gains

where the gains control the relative influence of each behavior.

Bird velocities and positions are updated using fixed-timestep numerical integration, subject to a maximum speed constraint.

## Spatial Partitioning

A uniform spatial grid partitions the simulation domain into cells. Neighbor searches examine nearby cells rather than comparing every bird against every other bird.

This reduces the number of distance calculations for typical spatial distributions and enables more efficient simulations at larger population sizes.

Periodic boundary conditions are applied to both bird positions and relative displacement calculations.

## Time Stepping

Simulation updates use a fixed timestep and an accumulator that tracks elapsed rendering time.

This decouples the numerical integration timestep from the rendering frame rate. A maximum frame-time contribution limits the amount of simulation work triggered by temporary rendering stalls.

## Performance

The project includes a benchmarking method for measuring average simulation step execution time.

Performance optimization has focused on:

- Eliminating redundant neighbor searches.
- Reusing allocated storage.
- Introducing spatial grid acceleration.
- Separating simulation computation from rendering.

Benchmark results should be collected using release builds and reported alongside hardware specifications, simulation parameters, and population sizes.

## Development Roadmap

Potential future improvements include:

- Additional automated tests and validation cases.
- Continuous integration with GitHub Actions.
- Expanded benchmarking and performance profiling.
- Runtime controls for population size and simulation parameters.
- Improved spatial grid handling and validation.
- Further optimization of memory allocation and neighbor-search performance.
- Life cycle, reproduction, and predator models.
- Sounds, colors, changing backgrounds.