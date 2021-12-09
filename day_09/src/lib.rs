use std::collections::BinaryHeap;

type Point = (usize, usize);

struct Grid<T> {
    data: Vec<Vec<T>>,
    shape: (usize, usize),
}

impl<T> Grid<T>
where
    T: Copy,
{
    fn new(data: Vec<Vec<T>>) -> Self {
        let shape = (data.len(), data[0].len());
        Self { shape, data }
    }

    fn at(&self, i: usize, j: usize) -> T {
        self.data[i][j]
    }

    fn get_neighbors(&self, i: usize, j: usize) -> Vec<(Point, T)> {
        let mut neighbors = Vec::with_capacity(4);

        if i > 0 {
            neighbors.push(((i - 1, j), self.at(i - 1, j)));
        }

        if j + 1 < self.shape.1 {
            neighbors.push(((i, j + 1), self.at(i, j + 1)));
        }

        if i + 1 < self.shape.0 {
            neighbors.push(((i + 1, j), self.at(i + 1, j)));
        }

        if j > 0 {
            neighbors.push(((i, j - 1), self.at(i, j - 1)));
        }

        neighbors
    }
}

fn parse_input(input: &str) -> Grid<u8> {
    let data = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Grid::new(data)
}

fn is_low_point(grid: &Grid<u8>, i: usize, j: usize) -> bool {
    let cur = grid.data[i][j];
    grid.get_neighbors(i, j).into_iter().all(|(_, x)| x > cur)
}

fn walk_basin(grid: &Grid<u8>, mask: &mut Grid<bool>, i: usize, j: usize) {
    if mask.at(i, j) || grid.at(i, j) == 9 {
        return;
    }

    mask.data[i][j] = true;

    grid.get_neighbors(i, j)
        .into_iter()
        .for_each(|((i, j), _)| walk_basin(grid, mask, i, j));
}

pub fn part_a(input: Option<&str>) -> u64 {
    let grid = parse_input(input.unwrap_or(include_str!("../input.txt")));

    let mut height = Vec::new();

    for i in 0..grid.shape.0 {
        for j in 0..grid.shape.1 {
            if is_low_point(&grid, i, j) {
                height.push(grid.at(i, j));
            }
        }
    }

    height.into_iter().map(|x| x as u64 + 1).sum::<u64>()
}

pub fn part_b(input: Option<&str>) -> u64 {
    let grid = parse_input(input.unwrap_or(include_str!("../input.txt")));

    let mut mask = Grid {
        shape: grid.shape,
        data: vec![vec![false; grid.shape.1]; grid.shape.0],
    };

    let mut basins = BinaryHeap::<u64>::new();

    for i in 0..grid.shape.0 {
        for j in 0..grid.shape.1 {
            if is_low_point(&grid, i, j) {
                walk_basin(&grid, &mut mask, i, j);
                let prev = basins.iter().sum::<u64>();
                basins.push(mask.data.iter().flatten().filter(|&&x| x).count() as u64 - prev);
            }
        }
    }

    (0..3).map(|_| basins.pop().unwrap()).product::<u64>()
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";

    #[test]
    fn test_part_1_works() {
        let result = super::part_a(Some(EXAMPLE));
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part_1() {
        let result = super::part_a(None);
        assert_eq!(result, 494);
    }

    #[test]
    fn test_part_2_works() {
        let result = super::part_b(Some(EXAMPLE));
        assert_eq!(result, 1134);
    }

    #[test]
    fn test_part_2() {
        let result = super::part_b(None);
        assert_eq!(result, 1048128);
    }
}
