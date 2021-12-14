use std::collections::HashMap;

type InsertionRules = HashMap<(u8, u8), u8>;

fn read_input(input: &str) -> (Vec<u8>, InsertionRules) {
    let mut lines = input.lines();

    let seq = lines.next().unwrap().as_bytes().to_vec();

    let rules = lines
        .into_iter()
        .skip(1)
        .map(|line| {
            let b = line.as_bytes();
            ((b[0], b[1]), b[6])
        })
        .collect::<HashMap<_, _>>();

    (seq, rules)
}

fn run_count(seq: &[u8], rules: &InsertionRules, n: usize) -> HashMap<u8, u64> {
    let mut pairs: HashMap<(u8, u8), u64> = seq.windows(2).fold(HashMap::new(), |mut acc, x| {
        *acc.entry((x[0], x[1])).or_insert(0) += 1;
        acc
    });

    let mut counter: HashMap<u8, u64> = seq.iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(*x).or_insert(0) += 1;
        acc
    });

    for _ in 0..n {
        let mut update = HashMap::new();
        for (pair, &middle) in rules {
            if let Some(&count) = pairs.get(pair) {
                *update.entry((pair.0, middle)).or_insert(0) += count;
                *update.entry((middle, pair.1)).or_insert(0) += count;
                *counter.entry(middle).or_insert(0) += count;
                pairs.remove(pair);
            }
        }
        // apply an update
        for (k, v) in update.into_iter() {
            *pairs.entry(k).or_insert(0) += v;
        }
    }

    counter
}

pub fn run_simulation(input: Option<&str>, n: usize) -> u64 {
    let (seq, rules) = read_input(input.unwrap_or(include_str!("../input.txt")));
    let counter = run_count(&seq, &rules, n);
    counter.values().max().unwrap() - counter.values().min().unwrap()
}

pub fn part_a(input: Option<&str>) -> u64 {
    run_simulation(input, 10)
}

pub fn part_b(input: Option<&str>) -> u64 {
    run_simulation(input, 40)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let (seq, rules) = super::read_input(include_str!("../example.txt"));

        assert_eq!(seq, b"NNCB");
        assert_eq!(rules.len(), 16);
        assert_eq!(rules.get(&(b'C', b'H')).unwrap(), &b'B');

        assert_eq!(super::run_count(&seq, &rules, 2).values().sum::<u64>(), 13);
        assert_eq!(super::run_count(&seq, &rules, 5).values().sum::<u64>(), 97);
    }

    #[test]
    fn test_part_a_works() {
        let result = super::part_a(Some(include_str!("../example.txt")));
        assert_eq!(result, 1588);
    }

    #[test]
    fn test_part_a() {
        let result = super::part_a(None);
        assert_eq!(result, 3230);
    }

    #[test]
    fn test_part_b_works() {
        let result = super::part_b(Some(include_str!("../example.txt")));
        assert_eq!(result, 2188189693529);
    }

    #[test]
    fn test_part_b() {
        let result = super::part_b(None);
        assert_eq!(result, 3542388214529);
    }
}
