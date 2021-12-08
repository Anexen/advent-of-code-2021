mod stats;

fn read_input(path: Option<&str>) -> Vec<i32> {
    std::fs::read_to_string(path.unwrap_or(&format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))))
        .unwrap()
        .trim_end()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn calculate_fuel(crabs: &Vec<i32>, alignment_point: i32) -> i32 {
    crabs.iter().map(|&x| (x - alignment_point).abs()).sum()
}

fn calculate_fuel_2(crabs: &Vec<i32>, alignment_point: i32) -> i32 {
    crabs
        .iter()
        .map(|&x| {
            let n = (x - alignment_point).abs();
            (2 + (n - 1)) * n / 2 // arithmetic progression
        })
        .sum()
}

pub fn part_a(path: Option<&str>) -> u64 {
    let input = read_input(path);
    let start = stats::median(&input).unwrap().round() as i32;
    calculate_fuel(&input, start) as u64
}

pub fn part_b(path: Option<&str>) -> u64 {
    let input = read_input(path);
    let start = stats::median(&input).unwrap().round() as i32;

    let mut best_score = calculate_fuel_2(&input, start);

    for direction in [-1, 1] {
        for i in 1..input.len() {
            let point = start + (i as i32) * direction;
            let score = calculate_fuel_2(&input, point);
            if score > best_score {
                break;
            } else {
                best_score = score;
            }
        }
    }

    best_score as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1_works() {
        let path = format!("{}/example.txt", env!("CARGO_MANIFEST_DIR"));
        assert_eq!(super::part_a(Some(&path)), 37);
    }

    #[test]
    fn test_part_1() {
        let path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
        assert_eq!(super::part_a(Some(&path)), 340052);
    }

    #[test]
    fn test_part_2_works() {
        let path = format!("{}/example.txt", env!("CARGO_MANIFEST_DIR"));
        assert_eq!(super::part_b(Some(&path)), 168);
    }

    #[test]
    fn test_part_2() {
        let path = format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"));
        assert_eq!(super::part_b(Some(&path)), 92948968);
    }
}
