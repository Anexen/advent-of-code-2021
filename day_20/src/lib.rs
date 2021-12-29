use std::collections::HashSet;

type Range = (i64, i64);

struct Image {
    pixels: HashSet<(i64, i64)>,
    x_range: Range,
    y_range: Range,
    default: bool,
}

impl Image {
    fn is_relevant(&self, x: i64, y: i64) -> bool {
        x >= self.x_range.0 && x <= self.x_range.1 && y >= self.y_range.0 && y <= self.y_range.1
    }

    fn iter_area(&self, x: i64, y: i64) -> impl Iterator<Item = bool> + '_ {
        AREA.iter()
            .map(move |(dx, dy)| (x + dx, y + dy))
            .map(|(x, y)| {
                if self.is_relevant(x, y) {
                    self.pixels.contains(&(x, y))
                } else {
                    self.default
                }
            })
    }
}

fn read_input(input: Option<&str>) -> (HashSet<i64>, Image) {
    let (algo, image) = input
        .unwrap_or(include_str!("../input.txt"))
        .split_once("\n\n")
        .unwrap();

    let is_lit = |(i, &x)| if x == b'#' { Some(i as i64) } else { None };

    let algo = algo
        .as_bytes()
        .iter()
        .enumerate()
        .filter_map(is_lit)
        .collect();

    let n_rows = image.as_bytes().iter().filter(|&&x| x == b'\n').count() - 1;
    let n_cols = image.split("\n").next().unwrap().len() - 1;

    println!("{} {}", n_rows, n_cols);

    let pixels = image
        .split("\n")
        .enumerate()
        .flat_map(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(is_lit)
                .map(move |j| (i as i64, j))
        })
        .collect();

    (
        algo,
        Image {
            pixels,
            x_range: (0, n_rows as i64),
            y_range: (0, n_cols as i64),
            default: false,
        },
    )
}

const AREA: [(i64, i64); 9] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn bits_to_num(bits: impl Iterator<Item = bool>) -> i64 {
    bits.fold(0, |acc, bit| (acc << 1) | (bit as i64))
}

fn enhance(algo: &HashSet<i64>, image: &Image) -> Image {
    let pixels = ((image.x_range.0 - 1)..=(image.x_range.1 + 1))
        .flat_map(|x| {
            ((image.y_range.0 - 1)..=(image.y_range.1 + 1)).filter_map(move |y| {
                let code = bits_to_num(image.iter_area(x, y));

                if algo.contains(&code) {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect();

    Image {
        pixels,
        x_range: (image.x_range.0 - 1, image.x_range.1 + 1),
        y_range: (image.y_range.0 - 1, image.y_range.1 + 1),
        default: algo.contains(if image.default { &0b111111111 } else { &0 }),
    }
}

pub fn part_a(input: Option<&str>) -> u64 {
    let (algo, mut image) = read_input(input);

    for _i in 0..2 {
        image = enhance(&algo, &image);
    }

    image.pixels.len() as u64
}

pub fn part_b(input: Option<&str>) -> u64 {
    let (algo, mut image) = read_input(input);

    for _i in 0..50 {
        image = enhance(&algo, &image);
    }

    println!("{}", image.pixels.len());

    image.pixels.len() as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a_works() {
        assert_eq!(super::part_a(Some(include_str!("../example.txt"))), 35);
    }

    #[test]
    fn test_part_a() {
        assert_eq!(super::part_a(None), 5663);
    }

    #[test]
    fn test_part_b_works() {
        assert_eq!(super::part_b(Some(include_str!("../example.txt"))), 3351);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(super::part_b(None), 19638);
    }
}
