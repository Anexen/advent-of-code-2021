pub fn part_a() -> u64 {
    include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .as_slice()
        .windows(2)
        .filter(|s| s[1] > s[0])
        .count() as u64
}

pub fn part_b() -> u64 {
    include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .as_slice()
        .windows(4)
        .filter(|&s| s[0..3].iter().sum::<i64>() < s[1..4].iter().sum())
        .count() as u64
}
