use std::collections::VecDeque;

use anyhow::Context;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1, newline, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    Parser,
};
use nom_supreme::ParserExt;

pub fn parse(
    input: &str,
) -> anyhow::Result<(VecDeque<VecDeque<char>>, Vec<(usize, usize, usize)>)> {
    let (input_state, moves) = common::parse(
        input,
        separated_pair(
            separated_list1(
                newline,
                separated_list1(
                    char(' '),
                    alt((
                        delimited(char('['), anychar, char(']')).map(Some),
                        tag("   ").value(None),
                    )),
                ),
            ),
            many1(alt((space1, digit1))).delimited_by(newline),
            separated_list1(
                newline,
                tuple((
                    digit1.parse_from_str::<usize>().preceded_by(tag("move ")),
                    digit1.parse_from_str::<usize>().preceded_by(tag(" from ")),
                    digit1.parse_from_str::<usize>().preceded_by(tag(" to ")),
                )),
            )
            .preceded_by(newline),
        )
        .terminated(newline.opt()),
    )?;

    let mut stacks = VecDeque::new();
    for line in input_state {
        for (index, item) in line.into_iter().enumerate() {
            if stacks.len() <= index {
                stacks.push_back(VecDeque::new());
            }
            if let Some(item) = item {
                stacks[index].push_front(item);
            }
        }
    }

    Ok((stacks, moves))
}

pub fn challenge1(input: &str) -> anyhow::Result<String> {
    let (mut stacks, moves) = parse(input)?;

    for (count, from, to) in moves {
        println!("moving {count} from {from} to {to} : {stacks:?}");
        for _ in 0..count {
            let tmp = stacks[from - 1].pop_back().context("Stack was empty")?;
            stacks[to - 1].push_back(tmp);
        }
    }

    Ok(stacks
        .into_iter()
        .filter_map(|mut stack| stack.pop_back())
        .collect())
}

pub fn challenge2(input: &str) -> anyhow::Result<String> {
    let (mut stacks, moves) = parse(input)?;

    for (count, from, to) in moves {
        println!("moving {count} from {from} to {to} : {stacks:?}");
        let pivot = stacks[from - 1].len() - count;
        while let Some(item) = stacks[from - 1].remove(pivot) {
            stacks[to - 1].push_back(item);
        }
    }

    Ok(stacks
        .into_iter()
        .filter_map(|mut stack| stack.pop_back())
        .collect())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");
    const SAMPLE: &str = indoc::indoc! {"
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2"};

    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (SAMPLE, "CMZ"),
            (INPUT, "RTGWZTHLD"),
        ];
        for (input, output) in expected {
            assert_eq!(
                crate::challenge1(input).unwrap(),
                output,
                "For input {}",
                input
            )
        }
        Ok(())
    }

    #[test]
    fn test_challenge2() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (SAMPLE, "MCD"),
            (INPUT, "STHGRZZFR"),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
