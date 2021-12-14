use std::collections::HashSet;

type Point = (u16, u16);

enum Fold {
    X(u16),
    Y(u16),
}

fn read_input(input: &str) -> (Vec<Point>, Vec<Fold>) {
    let points = input
        .lines()
        .map_while(|line| {
            if line.is_empty() {
                None
            } else {
                let (x, y) = line.split_once(",").unwrap();
                Some((x.parse().unwrap(), y.parse().unwrap()))
            }
        })
        .collect::<Vec<_>>();

    let folds = input
        .lines()
        .skip(points.len() + 1)
        .map(|line| match line.split_once("=").unwrap() {
            ("fold along x", x) => Fold::X(x.parse().unwrap()),
            ("fold along y", y) => Fold::Y(y.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect();

    (points, folds)
}

fn transform(p: Point, fold: &Fold) -> Point {
    match fold {
        Fold::X(x) if p.0 > *x => (x - (p.0 - x), p.1),
        Fold::Y(y) if p.1 > *y => (p.0, y - (p.1 - y)),
        _ => p,
    }
}

pub fn part_a(input: Option<&str>) -> u64 {
    let (points, folds) = read_input(input.unwrap_or(include_str!("../input.txt")));

    points
        .into_iter()
        .map(|p| transform(p, &folds[0]))
        .collect::<HashSet<_>>()
        .len() as u64
}

pub fn part_b(input: Option<&str>) -> u64 {
    let (points, folds) = read_input(input.unwrap_or(include_str!("../input.txt")));

    let points = points
        .into_iter()
        .map(|p| folds.iter().fold(p, transform))
        .collect::<HashSet<_>>();

    let shape = (
        points.iter().map(|p| p.0).max().unwrap(),
        points.iter().map(|p| p.1).max().unwrap(),
    );

    for y in 0..=shape.1 {
        for x in 0..=shape.0 {
            print!("{}", if points.contains(&(x, y)) { '#' } else { '.' })
        }
        println!();
    }

    0
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn test_folding_works() {
        let (points, folds) = super::read_input(include_str!("../example.txt"));

        let result = points
            .into_iter()
            .map(|p| folds.iter().fold(p, super::transform))
            .collect::<HashSet<_>>()
            .len();

        assert_eq!(result, 16);
    }

    #[test]
    fn test_part_a_works() {
        let result = super::part_a(Some(include_str!("../example.txt")));
        assert_eq!(result, 17);
    }
    #[test]
    fn test_part_a() {
        let result = super::part_a(None);
        assert_eq!(result, 795);
    }
}
