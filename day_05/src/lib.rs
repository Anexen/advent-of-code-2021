type Point = (usize, usize);
type Line = (Point, Point);

enum Direction {
    Up,
    Down,
}

fn read_input(input: Option<&str>) -> impl Iterator<Item = Line> + '_ {
    input
        .unwrap_or(include_str!("../input.txt"))
        .lines()
        .map(|line| {
            let mut s = line.split(" -> ").map(|pair| {
                let mut s = pair.split(",").map(|x| x.parse::<usize>().unwrap());
                (s.next().unwrap(), s.next().unwrap())
            });

            (s.next().unwrap(), s.next().unwrap())
        })
}

fn calculate_shape(input: &[Line]) -> (usize, usize) {
    let n = 1 + *input
        .iter()
        .map(|((x1, _), (x2, _))| x1.max(x2))
        .max()
        .unwrap();

    let m = 1 + *input
        .iter()
        .map(|((_, y1), (_, y2))| y1.max(y2))
        .max()
        .unwrap();

    (n, m)
}

fn abs_diff(a: &usize, b: &usize) -> usize {
    a.max(b) - a.min(b)
}

pub fn part_a(input: Option<&str>) -> u64 {
    let input = read_input(input)
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .collect::<Vec<_>>();

    let (n, m) = calculate_shape(&input);

    let mut grid = vec![vec![0u8; m]; n];

    for ((x1, y1), (x2, y2)) in input {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                grid[x1][y] += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                grid[x][y1] += 1;
            }
        }
    }

    grid.into_iter().flatten().filter(|&x| x > 1).count() as u64
}

pub fn part_b(input: Option<&str>) -> u64 {
    let input = read_input(input)
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2 || abs_diff(x1, x2) == abs_diff(y1, y2))
        .collect::<Vec<_>>();

    let (n, m) = calculate_shape(&input);

    let mut grid = vec![vec![0u8; m]; n];

    for ((x1, y1), (x2, y2)) in input {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                grid[x1][y] += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                grid[x][y1] += 1;
            }
        } else if abs_diff(&x1, &x2) == abs_diff(&y1, &y2) {
            let length = abs_diff(&x1, &x2);
            let (x_start, y_start, y_direction) = if x1 < x2 {
                if y1 < y2 {
                    (x1, y1, Direction::Up)
                } else {
                    (x1, y1, Direction::Down)
                }
            } else {
                if y1 < y2 {
                    (x2, y2, Direction::Down)
                } else {
                    (x2, y2, Direction::Up)
                }
            };

            match y_direction {
                Direction::Up => {
                    for i in 0..=length {
                        grid[x_start + i][y_start + i] += 1;
                    }
                }
                Direction::Down => {
                    for i in 0..=length {
                        grid[x_start + i][y_start - i] += 1;
                    }
                }
            }
        }
    }

    grid.into_iter().flatten().filter(|&x| x > 1).count() as u64
}
