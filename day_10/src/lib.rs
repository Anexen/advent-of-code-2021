use std::collections::VecDeque;
use std::ops::ControlFlow;

fn match_bracket(
    mut stack: VecDeque<char>,
    expected_prev: &char,
    score: u64,
) -> ControlFlow<u64, VecDeque<char>> {
    if stack.back().unwrap_or(&'-') != expected_prev {
        ControlFlow::Break(score)
    } else {
        stack.pop_back();
        ControlFlow::Continue(stack)
    }
}

fn fold_line(line: &str) -> ControlFlow<u64, VecDeque<char>> {
    line.chars().try_fold(VecDeque::new(), |mut a, x| match x {
        '(' | '[' | '{' | '<' => {
            a.push_back(x);
            ControlFlow::Continue(a)
        }
        ')' => match_bracket(a, &'(', 3),
        ']' => match_bracket(a, &'[', 57),
        '}' => match_bracket(a, &'{', 1197),
        '>' => match_bracket(a, &'<', 25137),
        _ => unreachable!(),
    })
}

pub fn part_a(input: Option<&str>) -> u64 {
    input
        .unwrap_or(include_str!("../input.txt"))
        .lines()
        .map(|line| match fold_line(line) {
            ControlFlow::Break(score) => score,
            ControlFlow::Continue(_) => 0,
        })
        .sum()
}

pub fn part_b(input: Option<&str>) -> u64 {
    let mut scores = input
        .unwrap_or(include_str!("../input.txt"))
        .lines()
        .map(|line| match fold_line(line) {
            ControlFlow::Break(_) => 0,
            ControlFlow::Continue(rest) => rest.iter().rev().fold(0, |a, x| {
                let score = match x {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
                a * 5 + score
            }),
        })
        .filter(|x| x != &0)
        .collect::<Vec<_>>();

    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a_works() {
        let result = super::part_a(Some(include_str!("../example.txt")));
        assert_eq!(result, 26397);
    }

    #[test]
    fn test_part_a() {
        let result = super::part_a(None);
        assert_eq!(result, 339411);
    }

    #[test]
    fn test_part_b_works() {
        let result = super::part_b(Some(include_str!("../example.txt")));
        assert_eq!(result, 288957);
    }

    #[test]
    fn test_part_b() {
        let result = super::part_b(None);
        assert_eq!(result, 2289754624);
    }
}
