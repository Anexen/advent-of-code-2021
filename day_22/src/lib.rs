use std::collections::HashSet;
use std::ops::RangeInclusive;

struct Cube {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
}

enum Operation {
    On(Cube),
    Off(Cube),
}

fn read_input(input: Option<&str>) -> Vec<Operation> {
    input
        .unwrap_or(include_str!("../input.txt"))
        .lines()
        .map(|line| {
            let (op, ranges) = line.split_once(" ").unwrap();
            let mut ranges = ranges.split(",").map(|r| {
                let (_, t) = r.split_once("=").unwrap();
                let (a, b) = t.split_once("..").unwrap();
                a.parse::<i64>().unwrap().max(-50)..=b.parse::<i64>().unwrap().min(50)
            });

            let cube = Cube {
                x: ranges.next().unwrap(),
                y: ranges.next().unwrap(),
                z: ranges.next().unwrap(),
            };
            if op == "on" {
                Operation::On(cube)
            } else {
                Operation::Off(cube)
            }
        })
        .collect()
}

pub fn part_a(input: Option<&str>) -> u64 {
    // Very bad. Try interval tree or define cuboid intersection algorithm
    // https://en.wikipedia.org/wiki/Interval_tree#Higher_dimensions
    let operations = read_input(input);
    let mut points = HashSet::new();

    for op in &operations {
        match op {
            Operation::On(cube) => {
                for x in cube.x.clone() {
                    for y in cube.y.clone() {
                        for z in cube.z.clone() {
                            points.insert((x, y, z));
                        }
                    }
                }
            }
            Operation::Off(cube) => {
                for x in cube.x.clone() {
                    for y in cube.y.clone() {
                        for z in cube.z.clone() {
                            points.remove(&(x, y, z));
                        }
                    }
                }
            }
        }
    }

    points.len() as u64
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

    #[test]
    fn test_part_a() {
        assert_eq!(super::part_a(None), 590784);
    }
}
