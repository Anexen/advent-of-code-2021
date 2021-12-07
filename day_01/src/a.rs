fn main() {
    let res = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .as_slice()
        .windows(2)
        .filter(|s| s[1] > s[0])
        .count();

    println!("{}", res);
}
