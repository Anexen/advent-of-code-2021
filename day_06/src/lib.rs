use std::collections::HashMap;

fn read_input(input: Option<&str>) -> HashMap<u8, u64> {
    let mut result = HashMap::with_capacity(8);

    input
        .unwrap_or(include_str!("../input.txt"))
        .trim_end()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .for_each(|x| *result.entry(x).or_insert(0) += 1);

    result
}

fn simulate(mut state: HashMap<u8, u64>, n: usize) -> u64 {
    for _ in 0..n {
        state = (0u8..9)
            .map(|t| {
                if t == 8 {
                    // lanternfish with internal timer of 0
                    // would create a new lanternfish with an internal timer of 8
                    (t, *state.get(&0).unwrap_or(&0u64))
                } else if t == 6 {
                    // lanternfish with internal timer of 0 would reset its timer to 6
                    let reset = *state.get(&0).unwrap_or(&0u64);
                    // + lanternfishes which decrease theirs timer from 7
                    let new = *state.get(&7).unwrap_or(&0u64);
                    (t, reset + new)
                } else {
                    // move lanternfishes to the next generation
                    (t, *state.get(&(t + 1)).unwrap_or(&0u64))
                }
            })
            .collect::<HashMap<_, _>>();
    }

    state.into_values().sum()
}

pub fn part_a(input: Option<&str>) -> u64 {
    let initial_state = read_input(input);
    simulate(initial_state, 80)
}

pub fn part_b(input: Option<&str>) -> u64 {
    let initial_state = read_input(input);
    simulate(initial_state, 256)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a_works() {
        assert_eq!(super::part_a(Some("3,4,3,1,2")), 5934);
    }

    #[test]
    fn test_part_a() {
        assert_eq!(super::part_a(None), 362639);
    }

    #[test]
    fn test_part_b_works() {
        assert_eq!(super::part_b(Some("3,4,3,1,2")), 26984457539);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(super::part_b(None), 1639854996917);
    }
}
