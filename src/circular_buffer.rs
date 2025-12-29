pub struct Grid<const H: usize, const D: usize>([[u16; H]; D]);

#[derive(Debug)]
pub struct CircularBuffer<'g, const N: usize, T> {
    grid: &'g [T],
    pub buffer: [&'g T; N],
    oldest_idx: usize,
    next_elm_idx: usize,
}

impl<'g, const N: usize, T> CircularBuffer<'g, N, T> {
    pub fn new(grid: &'g [T]) -> Self {
        assert!(grid.len() >= N);
        let mut buffer = [&grid[0]; N];
        for i in 0..N {
            let idx = ((i + grid.len()) - (N / 2)) % grid.len();
            buffer[i] = &grid[idx];
        }
        Self {
            grid,
            buffer,
            oldest_idx: 0,
            next_elm_idx: N - (N / 2),
        }
    }
}

impl<'g, const N: usize, T> Iterator for CircularBuffer<'g, N, T> {
    type Item = [&'g T; N];
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_elm_idx == N / 2 {
            return None;
        }
        self.buffer[self.oldest_idx] = &self.grid[self.next_elm_idx];
        self.oldest_idx = (self.oldest_idx + 1) % N;
        self.next_elm_idx = (self.next_elm_idx + 1) % self.grid.len();
        Some(self.buffer)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let data: [[u16; 2]; 5] = [[1, 2], [3, 4], [5, 6], [7, 8], [9, 10]];
        let buf = CircularBuffer::<3, _>::new(&data);
        println!("{:?}", buf.buffer);
        for x in buf.into_iter() {
            println!("{:?}", &x);
        }
    }
}
