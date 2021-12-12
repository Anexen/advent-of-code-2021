use std::collections::HashSet;

const NEIGHBORS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

struct Grid<T> {
    data: Vec<Vec<T>>,
    shape: (usize, usize),
}

impl<T> Grid<T> {
    fn from_vec(data: Vec<Vec<T>>) -> Self {
        let shape = (data.len(), data.get(0).unwrap_or(&Vec::new()).len());
        Self { data, shape }
    }

    fn get(&self, i: usize, j: usize) -> &T {
        &self.data[i][j]
    }

    fn set(&mut self, i: usize, j: usize, value: T) {
        self.data[i][j] = value
    }

    fn get_neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::with_capacity(8);
        let (i, j) = (i as i32, j as i32);
        for (di, dj) in NEIGHBORS {
            if i + di >= 0
                && i + di < self.shape.0 as i32
                && j + dj >= 0
                && j + dj < self.shape.1 as i32
            {
                neighbors.push(((i + di) as usize, (j + dj) as usize));
            }
        }
        neighbors
    }
}

impl Grid<u8> {
    pub fn simulate(&mut self, n: u32) -> u64 {
        (0..n).fold(0, |acc, _i| acc + self.step())
    }

    pub fn step(&mut self) -> u64 {
        let mut flushed: HashSet<(usize, usize)> = HashSet::new();
        // the energy level of each octopus increases by 1
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                self.inc(i, j);
                self.try_flush(i, j, &mut flushed);
            }
        }
        self.reset();
        flushed.len() as u64
    }

    fn try_flush(&mut self, i: usize, j: usize, flushed: &mut HashSet<(usize, usize)>) {
        if !flushed.contains(&(i, j)) && self.get(i, j) > &9 {
            flushed.insert((i, j));
            for (ni, nj) in self.get_neighbors(i, j) {
                self.inc(ni, nj);
                self.try_flush(ni, nj, flushed)
            }
        }
    }

    fn reset(&mut self) {
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                if self.get(i, j) > &9 {
                    self.set(i, j, 0);
                }
            }
        }
    }

    fn inc(&mut self, i: usize, j: usize) {
        self.set(i, j, self.get(i, j) + 1)
    }
}

fn read_input(input: &str) -> Grid<u8> {
    Grid::from_vec(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect(),
    )
}

pub fn part_a(input: Option<&str>) -> u64 {
    read_input(input.unwrap_or(include_str!("../input.txt"))).simulate(100)
}

pub fn part_b(input: Option<&str>) -> u64 {
    let mut grid = read_input(input.unwrap_or(include_str!("../input.txt")));

    for i in 1..1000 {
        grid.step();
        if grid.data.iter().flatten().all(|x| x == &0) {
            return i as u64;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_flushes() {
        let input = "11111\n19991\n19191\n19991\n11111";
        let expected = "34543\n40004\n50005\n40004\n34543";

        let mut grid = super::read_input(input);
        let flushes = grid.simulate(1);
        assert_eq!(grid.data, super::read_input(expected).data);
        assert_eq!(flushes, 9);
    }

    #[test]
    fn test_part_a_works() {
        let flushes = super::part_a(Some(include_str!("../example.txt")));
        assert_eq!(flushes, 1656);
    }

    #[test]
    fn test_part_b_works() {
        let step = super::part_b(Some(include_str!("../example.txt")));
        assert_eq!(step, 195);
    }

    #[test]
    fn test_part_b() {
        let step = super::part_b(None);
        assert_eq!(step, 220);
    }
}
