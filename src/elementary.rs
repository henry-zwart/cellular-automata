use rand::prelude::*;
use std::fmt::Display;

#[derive(Debug)]
pub struct Rule(pub u8);

impl Rule {
    pub fn apply(&self, neighborhood: &[u8; 3]) -> u8 {
        let idx = Rule::get_transition_idx(neighborhood);
        (self.0 >> idx) & 0b_00000001
    }

    pub fn get_transition_idx(neighborhood: &[u8; 3]) -> usize {
        let idx: u8 = neighborhood
            .iter()
            .enumerate()
            .map(|(i, s)| s * 2_u8.pow(2_u32 - i as u32))
            .sum();
        idx as usize
    }
}

impl From<u8> for Rule {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy)]
pub struct Grid<const W: usize>(pub [u8; W]);

impl<const W: usize> Grid<W> {
    pub fn new(state: [u8; W]) -> Self {
        Self(state)
    }

    pub fn random(rng: &mut impl RngCore) -> Self {
        let mut state = [0u8; W];
        for i in 0..W {
            state[i] = rng.gen_bool(1.0 / 2.0) as u8
        }
        Self::new(state)
    }

    pub fn apply(&self, rule: &Rule) -> Self {
        let mut new_state = self.0;
        for (i, neighborhood) in GridIterator::new(self).enumerate() {
            new_state[i] = rule.apply(&neighborhood)
        }
        Self::new(new_state)
    }
}

impl<const W: usize> Default for Grid<W> {
    fn default() -> Self {
        let mid = W.div_euclid(2);
        let mut state = [0_u8; W];
        state[mid] = 1;
        Self(state)
    }
}

impl<const W: usize> Display for Grid<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.0 {
            write!(
                f,
                "{}",
                match x {
                    0 => " ",
                    1 => "o",
                    _ => "_",
                }
            )?
        }
        Ok(())
    }
}

struct GridIterator<'g, const W: usize> {
    grid: &'g Grid<W>,
    index: usize,
}

impl<'g, const W: usize> GridIterator<'g, W> {
    fn new(grid: &'g Grid<W>) -> Self {
        Self { grid, index: 0 }
    }
}

impl<'g, const W: usize> Iterator for GridIterator<'g, W> {
    type Item = [u8; 3];

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.grid.0.len();
        if self.index == len || len < 3 {
            return None;
        }

        let result = [
            self.grid.0[(self.index + len - 1) % len],
            self.grid.0[self.index],
            self.grid.0[(self.index + 1) % len],
        ];
        self.index += 1;
        Some(result)
    }
}

pub fn run_evolution<const W: usize>(
    initial_state: Grid<W>,
    rule: &Rule,
    n_generations: usize,
) -> Vec<Grid<W>> {
    let mut states = Vec::with_capacity(n_generations);
    states.push(initial_state);

    let mut state = initial_state;
    for _ in 1..n_generations {
        state = state.apply(rule);
        states.push(state)
    }
    states
}
