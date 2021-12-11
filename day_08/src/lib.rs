use std::collections::HashMap;

//        -gfedcba
// a   => 00000001
// fe  => 00110000
// cdg => 01001100
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Signal(u8);

impl std::fmt::Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S(0b{:b})", self.0)
    }
}

impl From<&str> for Signal {
    fn from(s: &str) -> Self {
        let bitmask = s.chars().fold(0u8, |a, c| a | (1 << (c as u8 - 'a' as u8)));
        Self(bitmask)
    }
}

struct Entry {
    input: Vec<Signal>,
    output: Vec<Signal>,
}

struct Decoder {
    codes: HashMap<Signal, u8>,
}

impl Decoder {
    fn new(data: &Vec<Signal>) -> Self {
        let mut codes: HashMap<Signal, u8> = HashMap::with_capacity(10);
        let mut index: [u8; 10] = [0; 10];

        // find digits in specific order
        for digit in [1, 4, 7, 8, 6, 9, 0, 2, 3, 5] {
            // check only unknown digits
            for signal in data.iter().filter(|&s| !codes.contains_key(s)) {
                let count = signal.0.count_ones();

                let matches = match digit {
                    0 => count == 6, // the rest in [0, 6, 9] triple
                    1 => count == 2,
                    2 => count == 5 && (signal.0 | index[4]) == index[8], // 2|4 == 8
                    3 => count == 5 && (signal.0 & index[7]) == index[7], // 3&7 == 7, but 5&7 != 7
                    4 => count == 4,
                    5 => count == 5, // the rest in [2, 3, 5] triple
                    6 => count == 6 && (signal.0 | index[1]) != signal.0, // 6|1 != 6, but 9|1 == 9 and 0|1 == 0
                    7 => count == 3,
                    8 => count == 7,
                    9 => {
                        // 9.count_ones() - ((4|7)&9).count_ones() = 1
                        // 0.count_ones() - ((4|7)&0).count_ones() = 2
                        count == 6
                            && signal.0.count_ones()
                                - ((index[4] | index[7]) & signal.0).count_ones()
                                == 1
                    }
                    _ => unreachable!(),
                };

                if matches {
                    codes.insert(*signal, digit);
                    index[digit as usize] = signal.0;
                    break;
                }
            }
            println!("{:?}", codes);
        }

        Self { codes }
    }

    fn decode(&self, signal: &Signal) -> u16 {
        *self.codes.get(signal).unwrap() as u16
    }

    fn decode_line(&self, signals: &Vec<Signal>) -> u16 {
        signals
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| 10_u16.pow(i as u32) * self.decode(x))
            .sum::<u16>() as u16
    }
}

fn parse_signals(signals: &str) -> Vec<Signal> {
    signals
        .split_whitespace()
        .map(|x| x.into())
        .collect::<Vec<_>>()
}

fn parse_input<'a>(input: &'a str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let mut s = line.split(" | ");
            let input = parse_signals(s.next().unwrap());
            let output = parse_signals(s.next().unwrap());
            Entry { input, output }
        })
        .collect::<Vec<_>>()
}

pub fn part_a(input: Option<&str>) -> u64 {
    let data = parse_input(input.unwrap_or(include_str!("../input.txt")));

    data.into_iter()
        .map(|e| {
            e.output
                .iter()
                .filter(|x| [2, 3, 4, 7].contains(&x.0.count_ones()))
                .count()
        })
        .sum::<usize>() as u64
}

pub fn part_b(input: Option<&str>) -> u64 {
    let data = parse_input(input.unwrap_or(include_str!("../input.txt")));

    data.into_iter()
        .map(|e| Decoder::new(&e.input).decode_line(&e.output) as u64)
        .sum::<u64>()
}

#[cfg(test)]
mod tests {

    use super::Signal;

    #[test]
    fn test_parse_signal_from_str() {
        assert_eq!(Signal::from("abc").0, 0b0000111);
        assert_eq!(Signal::from("cdf").0, 0b0101100);
    }

    #[test]
    fn test_decode() {
        let input = super::parse_input(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let line = input.first().unwrap();
        let decoder = super::Decoder::new(&line.input);

        assert_eq!(decoder.decode(&Signal::from("acedgfb")), 8);
        assert_eq!(decoder.decode(&Signal::from("cdfbe")), 5);
        assert_eq!(decoder.decode(&Signal::from("gcdfa")), 2);
        assert_eq!(decoder.decode(&Signal::from("fbcad")), 3);
        assert_eq!(decoder.decode(&Signal::from("dab")), 7);
        assert_eq!(decoder.decode(&Signal::from("cefabd")), 9);
        assert_eq!(decoder.decode(&Signal::from("cdfgeb")), 6);
        assert_eq!(decoder.decode(&Signal::from("eafb")), 4);
        assert_eq!(decoder.decode(&Signal::from("cagedb")), 0);
        assert_eq!(decoder.decode(&Signal::from("ab")), 1);

        assert_eq!(decoder.decode_line(&line.output), 5353);
    }

    #[test]
    fn test_part_a_works() {
        let result = super::part_a(Some(include_str!("../example.txt")));
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part_a() {
        let result = super::part_a(None);
        assert_eq!(result, 303);
    }

    #[test]
    fn test_part_b_works() {
        let result = super::part_b(Some(include_str!("../example.txt")));
        assert_eq!(result, 61229);
    }

    #[test]
    fn test_part_b() {
        let result = super::part_b(None);
        assert_eq!(result, 961734);
    }
}
