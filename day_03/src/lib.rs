pub const BITS: usize = 12;

pub fn read_numbers(input: Option<&str>) -> Vec<usize> {
    input
        .unwrap_or(include_str!("../input.txt"))
        .lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>()
}

pub fn find_most_common(numbers: &Vec<usize>) -> usize {
    bits_to_num(&find_most_common_bits(numbers))
}

fn count_ones(numbers: &Vec<usize>) -> [usize; BITS] {
    numbers.iter().fold([0; BITS], |mut counts, &x| {
        for i in 0..BITS {
            counts[BITS - i - 1] += get_bit_at_position(x, i)
        }
        counts
    })
}

fn find_most_common_bits(numbers: &Vec<usize>) -> Vec<usize> {
    let ones = count_ones(numbers);
    let n = numbers.len();

    ones.iter()
        .map(|&count| if count >= (n - count) { 1 } else { 0 })
        .collect()
}

fn bits_to_num(bits: &[usize]) -> usize {
    bits.iter().fold(0, |acc, &bit| (acc << 1) | bit)
}

fn get_bit_at_position(num: usize, pos: usize) -> usize {
    (num & (1 << pos)) >> pos
}

pub fn part_a(input: Option<&str>) -> u64 {
    let numbers = read_numbers(input);
    let gamma = find_most_common(&numbers);
    // invert lower 12 bits
    let epsilon = gamma ^ ((1 << 12) - 1);

    println!("gamma: {}, eps: {}", gamma, epsilon);
    println!("{}", gamma * epsilon);

    (gamma * epsilon) as u64
}

pub fn part_b(input: Option<&str>) -> u64 {
    let numbers = read_numbers(input);

    let mut oxygen = numbers.clone();
    for i in (0..BITS).rev() {
        let mode = find_most_common(&oxygen);

        oxygen = oxygen
            .into_iter()
            .filter(|&x| (x & (1 << i)) == (mode & (1 << i)))
            .collect();

        if oxygen.len() == 1 {
            break;
        }
    }

    let mut co2 = numbers;
    for i in (0..BITS).rev() {
        let mode = find_most_common(&co2);

        co2 = co2
            .into_iter()
            .filter(|&x| (x & (1 << i)) != (mode & (1 << i)))
            .collect();

        if co2.len() == 1 {
            break;
        }
    }

    (oxygen[0] * co2[0]) as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a() {
        assert_eq!(super::part_a(None), 775304);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(super::part_b(None), 1370737);
    }
}
