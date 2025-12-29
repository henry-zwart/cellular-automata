use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use cellular_automata::cli::{parse_cli, Initialisation};
use cellular_automata::elementary::{run_evolution, Grid};
use cellular_automata::plot::plot_evolution;

fn main() {
    let params = parse_cli();

    let grid: Grid<1024> = match params.init {
        Initialisation::Random => {
            let mut rng = ChaCha8Rng::seed_from_u64(params.seed);
            Grid::random(&mut rng)
        }
        Initialisation::Middle => Grid::default(),
    };

    let rule = params.rule.into();
    let n_generations = params.generations;

    let states = run_evolution(grid, &rule, n_generations);

    let _ = plot_evolution(&states, "automaton2.png");
}
