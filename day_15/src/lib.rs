use pathfinding::prelude::dijkstra;
use utils::{Grid, Neighbors};

pub fn part_a(input: Option<&str>) -> u64 {
    let grid = Grid::<u8>::from_str(input.unwrap_or(include_str!("../input.txt")));

    let goal = (grid.shape.0 - 1, grid.shape.1 - 1);
    let result = dijkstra(
        &(0, 0),
        |&(i, j)| {
            grid.get_neighbors(i, j, Neighbors::Quadratic)
                .into_iter()
                .map(|(ii, jj)| ((ii, jj), *grid.get(ii, jj) as u64))
                // .flatten()
                .collect::<Vec<_>>()
        },
        |p| *p == goal,
    );

    result.unwrap().1
}

pub fn part_b(input: Option<&str>) -> u64 {
    let grid = Grid::<u8>::from_str(input.unwrap_or(include_str!("../input.txt")));

    let goal = ((grid.shape.0 * 5 - 1) as i32, (grid.shape.1 * 5 - 1) as i32);
    let neighbors: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    let result = dijkstra(
        &(0, 0),
        |&(i, j)| {
            neighbors
                .iter()
                .filter_map(|(di, dj)| {
                    if i + di >= 0
                        && i + di < (grid.shape.0 * 5) as i32
                        && j + dj >= 0
                        && j + dj < (grid.shape.1 * 5) as i32
                    {
                        Some(((i + di), (j + dj)))
                    } else {
                        None
                    }
                })
                .map(|(ii, jj)| {
                    let weight =
                        *grid.get(ii as usize % grid.shape.0, jj as usize % grid.shape.1) as i32;
                    // correction
                    let weight = weight + ii / grid.shape.0 as i32 + jj / grid.shape.1 as i32 - 1;
                    ((ii, jj), (weight % 9 + 1) as u64)
                })
                // .flatten()
                .collect::<Vec<_>>()
        },
        |p| *p == goal,
    );

    result.unwrap().1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a_works() {
        let result = super::part_a(Some(include_str!("../example.txt")));
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_a() {
        let result = super::part_a(None);
        assert_eq!(result, 604);
    }

    #[test]
    fn test_part_b_works() {
        let result = super::part_b(Some(include_str!("../example.txt")));
        assert_eq!(result, 315);
    }

    #[test]
    fn test_part_b() {
        let result = super::part_b(None);
        assert_eq!(result, 2907);
    }
}
