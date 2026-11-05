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

Each bird follows three local interaction rules:

1. **Separation**: Steer away from nearby birds to reduce crowding.
2. **Alignment**: Adjust velocity toward the average velocity of neighboring birds.
3. **Cohesion**: Steer toward the local center of neighboring birds.

The acceleration of bird $i$ is calculated as:

```math
\mathbf{a}_i =
k_s\mathbf{f}_{s,i}
+ k_a\mathbf{f}_{a,i}
+ k_c\mathbf{f}_{c,i}
```

where the gains control the relative influence of each behavior.

Bird velocities and positions are updated using fixed-timestep numerical integration, subject to a maximum speed constraint.

### Spatial Partitioning

A uniform spatial grid partitions the simulation domain into cells. Neighbor searches examine nearby cells rather than comparing every bird against every other bird.

This reduces the number of distance calculations for typical spatial distributions and enables more efficient simulations at larger population sizes.

Periodic boundary conditions are applied to both bird positions and relative displacement calculations.

### Time Stepping

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
