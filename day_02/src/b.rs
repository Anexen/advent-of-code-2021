fn main() {
    let (position, depth, _) = include_str!("../input.txt")
        .lines()
        .map(|line| line.split(" ").collect::<Vec<_>>())
        .map(|x| (x[0], x[1].parse::<i64>().unwrap()))
        .fold(
            (0, 0, 0),
            |(position, depth, aim), (direction, level)| match direction {
                "forward" => (position + level, depth + (aim * level), aim),
                "down" => (position, depth, aim + level),
                "up" => (position, depth, aim - level),
                _ => unimplemented!(),
            },
        );

    println!("{}", position * depth);
}
