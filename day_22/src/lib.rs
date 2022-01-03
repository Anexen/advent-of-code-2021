use intervaltree::IntervalTree;
use rstar::primitives::Rectangle;
use rstar::{RTree, AABB};

fn read_input(input: Option<&str>) -> RTree<Rectangle<(i64, i64, i64)>> {
    input
        .unwrap_or(include_str!("../input.txt"))
        .lines()
        .fold(RTree::new(), |mut acc, line| {
            let (op, ranges) = line.split_once(" ").unwrap();
            let ranges = ranges
                .split(",")
                .map(|r| {
                    let (_, t) = r.split_once("=").unwrap();
                    let (a, b) = t.split_once("..").unwrap();
                    (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
                })
                .collect::<Vec<_>>();

            let c1 = (ranges[0].0, ranges[1].0, ranges[2].0);
            let c2 = (ranges[0].1, ranges[1].1, ranges[2].1);
            let rect = Rectangle::from_corners(c1, c2);

            println!("{} {:?}", op, rect);

            if op == "on" {
                acc.insert(rect);
            } else {
                acc.drain_in_envelope(AABB::from_corners(c1, c2));
            }

            println!("{:?}", acc.root());
            println!("---------");
            acc
        })
}

pub fn part_a2() {
    let tree = [
        (0..3, 1),
        (1..4, 2),
        (2..5, 3),
        (3..6, 4),
        (4..7, 5),
        (5..8, 6),
        (4..5, 7),
        (2..7, 8),
    ]
    .iter()
    .cloned()
    .collect::<IntervalTree<u32, u32>>();
    println!("{:?}", tree);
}

pub fn part_a(input: Option<&str>) -> u64 {
    part_a2();
    let tree = read_input(input);
    tree.locate_in_envelope(&AABB::from_corners((-50i64, -50, -50), (50i64, 50, 50)))
        .map(|r| {
            let c1 = r.lower();
            let c2 = r.upper();
            (c2.0 - c1.0) * (c2.1 - c1.1) * (c2.2 - c1.2)
        })
        .sum::<i64>()
        .try_into()
        .unwrap()
}

pub fn part_b(input: Option<&str>) -> u64 {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a_works() {
        assert_eq!(super::part_a(Some(include_str!("../example.txt"))), 590784);
    }

    // #[test]
    // fn test_part_a() {
    //     assert_eq!(super::part_a(None), 590784);
    // }
}
