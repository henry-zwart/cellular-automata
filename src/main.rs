use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use cellular_automata::cli::parse_cli;
use cellular_automata::elementary::{run_evolution, Grid};
use cellular_automata::plot::plot_evolution;

fn main() {
    let params = parse_cli();

    let mut rng = ChaCha8Rng::seed_from_u64(params.seed);
    let grid: Grid<256> = Grid::random(&mut rng);

    let rule = params.rule.into();
    let n_generations = params.generations;

    let states = run_evolution(grid, &rule, n_generations);

    let _ = plot_evolution(&states, "automaton.png");
}
