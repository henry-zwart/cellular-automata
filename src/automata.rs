//use rand::prelude::*;
//use std::fmt::Display;
//
//#[derive(Debug)]
//pub struct Rule(pub u8);
//
//impl Rule {
//    pub fn apply(&self, neighborhood: u16, n_neighbors: usize) -> u8 {
//        let idx = Rule::get_transition_idx(neighborhood, n_neighbors);
//        (self.0 >> idx) & 1
//    }
//
//    pub fn get_transition_idx(neighborhood: u16, n_neighbors: usize) -> usize {
//        let mut idx = 0;
//        for i in 0..n_neighbors {
//            idx += (neighborhood >> (i * 3)) & 0b1111
//        }
//        idx as usize
//    }
//}
//
//impl From<u8> for Rule {
//    fn from(value: u8) -> Self {
//        Self(value)
//    }
//}
//
//#[derive(Clone, Copy)]
//pub struct Grid {
//    width: usize,
//    colours: usize,
//    value: u16,
//}
//
//impl Grid {
//    pub fn new(value: u16, width: usize, colours: usize) -> Self {
//        Self {
//            width,
//            colours,
//            value,
//        }
//    }
//
//    pub fn new_with_default_mid(width: usize, colours: usize) -> Self {
//        let mid = width.div_euclid(2);
//        let state = 1 << (mid * 3);
//        Self::new(state, width, colours)
//    }
//
//    pub fn random(width: usize, colours: usize, rng: &mut impl RngCore) -> Self {
//        let mut state = 0_u16;
//        for _ in 0..width {
//            state = (state << 3) + rng.gen_range(0..colours) as u16;
//        }
//        Self::new(state, width, colours)
//    }
//
//    //pub fn apply(&self, rule: &Rule) -> Self {
//    //    let mut new_state = self.0;
//    //    for (i, neighborhood) in GridIterator::new(self).enumerate() {
//    //        new_state[i] = rule.apply(&neighborhood)
//    //    }
//    //    Self::new(new_state)
//    //}
//}
//
//impl Display for Grid {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        for i in self.width..=0 {
//            write!(f, "{}", (self.value >> (self.width - i)) & 0b1111)?
//        }
//        Ok(())
//    }
//}
//
//struct GridIterator<'g> {
//    grid: &'g Grid,
//    index: usize,
//}
//
//impl<'g> GridIterator<'g> {
//    fn new(grid: &'g Grid) -> Self {
//        Self { grid, index: 0 }
//    }
//}
//
//impl<'g> Iterator for GridIterator<'g> {
//    type Item = [u8; 3];
//
//    fn next(&mut self) -> Option<Self::Item> {
//        if self.index == self.grid.width || self.grid.width < 3 {
//            return None;
//        }
//
//        let result = [
//            ((self.grid.value >> (((self.index + self.grid.width - 1) % self.grid.width) * 3))
//                & 0b1111) as u8,
//            ((self.grid.value >> (self.index * 3)) & 0b1111) as u8,
//            ((self.grid.value >> (((self.index + 1) % self.grid.width) * 3)) & 0b1111) as u8,
//        ];
//        self.index += 1;
//        Some(result)
//    }
//}
//
//pub fn run_evolution(initial_state: Grid, rule: &Rule, n_generations: usize) -> Vec<Grid> {
//    let mut states = Vec::with_capacity(n_generations);
//    states.push(initial_state);
//
//    let mut state = initial_state;
//    for _ in 1..n_generations {
//        state = state.apply(rule);
//        states.push(state)
//    }
//    states
//}
