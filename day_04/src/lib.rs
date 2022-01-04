use std::cell::RefCell;

const N: usize = 5;

#[derive(Debug)]
pub struct Board {
    pub nums: Vec<Vec<i64>>,
}

impl Board {
    pub fn set(&mut self, value: i64) {
        self.nums.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|x| {
                if *x == value {
                    *x = -1
                }
            })
        })
    }

    pub fn is_winner(&self) -> bool {
        if self
            .nums
            .iter()
            .map(|row| row.iter().sum::<i64>())
            .any(|x| x == -(N as i64))
        {
            true
        } else if (0..N)
            .map(|j| self.nums.iter().map(|row| row[j]).sum::<i64>())
            .any(|x| x == -(N as i64))
        {
            true
        } else {
            false
        }
    }
    pub fn get_score(&self) -> i64 {
        self.nums
            .iter()
            .map(|row| row.iter().filter(|&x| x != &-1).sum::<i64>())
            .sum()
    }
}

#[derive(Debug)]
pub struct Bingo {
    pub order: Vec<i64>,
    pub boards: Vec<RefCell<Board>>,
}

pub fn read_input(input: Option<&str>) -> Bingo {
    let mut lines = input.unwrap_or(include_str!("../input.txt")).lines();
    let order = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let lines = lines.filter(|line| line.len() > 0).collect::<Vec<_>>();

    let boards = lines
        .chunks(N)
        .map(|lines| {
            lines
                .iter()
                .map(|l| {
                    l.split_whitespace()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .map(|b| RefCell::new(Board { nums: b }))
        .collect();

    Bingo { order, boards }
}

pub fn part_a(input: Option<&str>) -> u64 {
    let mut bingo = read_input(input);
    let mut last_winner_score: i64 = -1;

    for value in bingo.order.into_iter() {
        bingo.boards.retain(|board| {
            board.borrow_mut().set(value);
            let winner = board.borrow().is_winner();
            if winner {
                last_winner_score = board.borrow().get_score();
            }
            !winner
        });

        if bingo.boards.len() == 0 {
            return (last_winner_score * value) as u64;
        }
    }
    unreachable!()
}

pub fn part_b(input: Option<&str>) -> u64 {
    let bingo = read_input(input);

    // find first winner
    for value in bingo.order.into_iter() {
        for board in &bingo.boards {
            board.borrow_mut().set(value);
            if board.borrow().is_winner() {
                return (board.borrow().get_score() * value) as u64;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a_works() {
        assert_eq!(super::part_a(None), 123);
    }

    #[test]
    fn test_part_b_works() {
        assert_eq!(super::part_b(None), 123);
    }
}
