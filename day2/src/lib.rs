use nom::{character::complete::space1, error::ParseError, sequence::separated_pair, IResult};
use nom_supreme::error::ErrorTree;
use std::cmp::Ordering;

pub fn challenge1(input: &str) -> anyhow::Result<i64> {
    input.lines().try_fold(0, |mut acc, line| {
        let (opponent, yours) =
            common::parse(line, separated_pair(opponent_hand, space1, your_hand))?;

        if opponent < yours {
            acc += 6;
        } else if opponent == yours {
            acc += 3;
        }

        Ok(acc + i64::from(yours))
    })
}

pub fn challenge2(input: &str) -> anyhow::Result<i64> {
    input.lines().try_fold(0, |mut acc, line| {
        let (opponent, outcome) =
            common::parse(line, separated_pair(opponent_hand, space1, outcome))?;

        let yours = opponent >> outcome;

        if opponent < yours {
            acc += 6;
        } else if opponent == yours {
            acc += 3;
        }

        Ok(acc + i64::from(yours))
    })
}

fn opponent_hand(input: &str) -> IResult<&'_ str, Hand, ErrorTree<&'_ str>> {
    let (letter, remainder) = input.split_at(1);
    match letter {
        "A" => Ok((remainder, Hand::Rock)),
        "B" => Ok((remainder, Hand::Paper)),
        "C" => Ok((remainder, Hand::Scissors)),
        // This error isn't quite right but ignoring this for now.
        _ => Err(nom::Err::Error(ErrorTree::from_char(input, 'A'))),
    }
}

fn your_hand(input: &str) -> IResult<&'_ str, Hand, ErrorTree<&'_ str>> {
    let (letter, remainder) = input.split_at(1);
    match letter {
        "X" => Ok((remainder, Hand::Rock)),
        "Y" => Ok((remainder, Hand::Paper)),
        "Z" => Ok((remainder, Hand::Scissors)),
        // This error isn't quite right but ignoring this for now.
        _ => Err(nom::Err::Error(ErrorTree::from_char(input, 'X'))),
    }
}

fn outcome(input: &str) -> IResult<&'_ str, Outcome, ErrorTree<&'_ str>> {
    let (letter, remainder) = input.split_at(1);
    match letter {
        "X" => Ok((remainder, Outcome::Lose)),
        "Y" => Ok((remainder, Outcome::Draw)),
        "Z" => Ok((remainder, Outcome::Win)),
        // This error isn't quite right but ignoring this for now.
        _ => Err(nom::Err::Error(ErrorTree::from_char(input, 'X'))),
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Hand::Paper, Hand::Rock) => Some(Ordering::Greater),
            (Hand::Rock, Hand::Paper) => Some(Ordering::Less),
            (Hand::Rock, Hand::Scissors) => Some(Ordering::Greater),
            (Hand::Scissors, Hand::Rock) => Some(Ordering::Less),
            (Hand::Scissors, Hand::Paper) => Some(Ordering::Greater),
            (Hand::Paper, Hand::Scissors) => Some(Ordering::Less),
            (Hand::Rock, Hand::Rock) => Some(Ordering::Equal),
            (Hand::Paper, Hand::Paper) => Some(Ordering::Equal),
            (Hand::Scissors, Hand::Scissors) => Some(Ordering::Equal),
        }
    }
}

impl std::ops::Shr<usize> for Hand {
    type Output = Hand;

    fn shr(mut self, rhs: usize) -> Self::Output {
        for _ in 0..rhs {
            self = match self {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissors,
                Hand::Scissors => Hand::Rock,
            }
        }

        self
    }
}

impl std::ops::Shr<Outcome> for Hand {
    type Output = Hand;

    fn shr(self, rhs: Outcome) -> Self::Output {
        match rhs {
            Outcome::Win => self >> 1,
            Outcome::Draw => self,
            Outcome::Lose => self >> 2,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl From<Hand> for i64 {
    fn from(hand: Hand) -> i64 {
        match hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (
                indoc::indoc! {"
                    A Y
                    B X
                    C Z"
                },
                15,
            ),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge1(input)?, output, "For input {}", input)
        }
        Ok(())
    }

    #[test]
    fn test_challenge2() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (
                indoc::indoc! {"
                    A Y
                    B X
                    C Z"
                },
                12,
            ),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
