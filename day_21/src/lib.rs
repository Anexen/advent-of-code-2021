pub fn part_a(input: Option<(u64, u64)>) -> u64 {
    let (mut p1, mut p2) = input.unwrap_or((9, 3));

    p1 -= 1;
    p2 -= 1;

    let mut p1_score = 0;
    let mut p2_score = 0;

    for (i, s) in (1..u64::max_value()).step_by(3).enumerate() {
        let r = (s % 100) + (s + 1) % 100 + (s + 2) % 100;
        println!("{} {} {}", s, s + 1, s + 2);
        if i % 2 == 0 {
            p1 = (p1 + r) % 10;
            p1_score += p1 + 1;
            // println!("{}", p1_score);
        } else {
            p2 = (p2 + r) % 10;
            p2_score += p2 + 1;
            // println!("{}", p2_score);
        }
        if p1_score >= 1000 || p2_score >= 1000 {
            return p1_score.min(p2_score) * (i as u64 + 1) * 3;
        }
    }
    unreachable!();
}

pub fn part_b(input: Option<(u8, u8)>) -> u64 {
    let (p1, p2) = input.unwrap_or((9, 3));
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a_works() {
        assert_eq!(super::part_a(Some((4, 8))), 739785);
    }

    #[test]
    fn test_part_a() {
        assert_eq!(super::part_a(None), 1073709);
    }
}
