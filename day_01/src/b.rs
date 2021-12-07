fn main() {
    let res = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .as_slice()
        .windows(4)
        .filter(|&s| s[0..3].iter().sum::<i64>() < s[1..4].iter().sum())
        .count();

    println!("{}", res);
}
