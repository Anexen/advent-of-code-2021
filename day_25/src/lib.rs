use utils::Grid;

fn read_input(input: Option<&str>) -> Grid<u8> {
    Grid::from_vec(
        input
            .unwrap_or(include_str!("../input.txt"))
            .lines()
            .map(|line| line.bytes().collect())
            .collect(),
    )
}

fn apply_changes(grid: &mut Grid<u8>, changes: &Vec<(u8, usize, usize)>) {
    for &(cell, i, j) in changes {
        match cell {
            b'>' => {
                grid.set(i, j, b'.');
                grid.set(i, (j + 1) % grid.shape.1, b'>');
            }
            b'v' => {
                grid.set(i, j, b'.');
                grid.set((i + 1) % grid.shape.0, j, b'v');
            }
            _ => {
                unreachable!();
            }
        }
    }
}

pub fn part_a(input: Option<&str>) -> u64 {
    let mut grid = read_input(input);
    let mut changes = Vec::new();
    let mut step = 0;
    let mut changes_count = 0;

    loop {
        for i in 0..grid.shape.0 {
            for j in 0..grid.shape.1 {
                let cell = grid.get(i, j);
                match cell {
                    b'>' => {
                        if *grid.get(i, (j + 1) % grid.shape.1) == b'.' {
                            changes.push((b'>', i, j))
                        }
                    }
                    _ => {}
                }
            }
        }

        apply_changes(&mut grid, &changes);
        changes_count = changes.len();
        changes.clear();

        for i in 0..grid.shape.0 {
            for j in 0..grid.shape.1 {
                let cell = grid.get(i, j);
                match cell {
                    b'v' => {
                        if *grid.get((i + 1) % grid.shape.0, j) == b'.' {
                            changes.push((b'v', i, j))
                        }
                    }
                    _ => {}
                }
            }
        }

        apply_changes(&mut grid, &changes);
        changes_count += changes.len();
        changes.clear();

        step += 1;

        if changes_count == 0 {
            return step;
        }
    }
}

pub fn part_b(input: Option<&str>) -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a_works() {
        assert_eq!(super::part_a(Some(include_str!("../example.txt"))), 58);
    }

    #[test]
    fn test_part_a() {
        assert_eq!(super::part_a(None), 300);
    }
}
