use std::collections::HashSet;
use utils::{Grid, Neighbors};

fn simulate(grid: &mut Grid<u8>, n: u32) -> u64 {
    (0..n).fold(0, |acc, _i| acc + step(grid))
}

fn step(grid: &mut Grid<u8>) -> u64 {
    let mut flushed: HashSet<(usize, usize)> = HashSet::new();
    // the energy level of each octopus increases by 1
    for i in 0..grid.shape.0 {
        for j in 0..grid.shape.1 {
            inc(grid, i, j);
            try_flush(grid, i, j, &mut flushed);
        }
    }
    reset(grid);
    flushed.len() as u64
}

fn try_flush(grid: &mut Grid<u8>, i: usize, j: usize, flushed: &mut HashSet<(usize, usize)>) {
    if !flushed.contains(&(i, j)) && grid.get(i, j) > &9 {
        flushed.insert((i, j));
        for (ni, nj) in grid.get_neighbors(i, j, Neighbors::Diagonal) {
            inc(grid, ni, nj);
            try_flush(grid, ni, nj, flushed)
        }
    }
}

fn reset(grid: &mut Grid<u8>) {
    for i in 0..grid.shape.0 {
        for j in 0..grid.shape.1 {
            if grid.get(i, j) > &9 {
                grid.set(i, j, 0);
            }
        }
    }
}

fn inc(grid: &mut Grid<u8>, i: usize, j: usize) {
    grid.set(i, j, grid.get(i, j) + 1)
}

fn read_input(input: &str) -> Grid<u8> {
    Grid::<u8>::from_str(input)
}

pub fn part_a(input: Option<&str>) -> u64 {
    let mut grid = read_input(input.unwrap_or(include_str!("../input.txt")));
    simulate(&mut grid, 100)
}

pub fn part_b(input: Option<&str>) -> u64 {
    let mut grid = read_input(input.unwrap_or(include_str!("../input.txt")));

    for i in 1..1000 {
        step(&mut grid);
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
        let flushes = super::simulate(&mut grid, 1);
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
