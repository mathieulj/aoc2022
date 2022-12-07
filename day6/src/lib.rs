use std::collections::BTreeSet;

use anyhow::{bail, Context};
use itertools::Itertools;

pub fn challenge1(input: &str) -> anyhow::Result<usize> {
    input
        .char_indices()
        .tuple_windows()
        .find_map(|((_, a), (_, b), (_, c), (index, d))| {
            if a == b || a == c || a == d || b == c || b == d || c == d {
                None
            } else {
                Some(index + 1)
            }
        })
        .context("None found")
}

pub fn challenge2(input: &str) -> anyhow::Result<usize> {
    let mut buffer = ['a'; 14];

    for (index, char) in input.char_indices() {
        buffer[index % buffer.len()] = char;

        if index < buffer.len() - 1 {
            continue;
        }

        // There are ways to avoid this allocation but this is easy and good enough
        if buffer.iter().collect::<BTreeSet<_>>().len() == buffer.len() {
            return Ok(index + 1);
        }
    }

    bail!("None found")
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");
    const SAMPLE1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const SAMPLE2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const SAMPLE3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const SAMPLE4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const SAMPLE5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (SAMPLE1, 7),
            (SAMPLE2, 5),
            (SAMPLE3, 6),
            (SAMPLE4, 10),
            (SAMPLE5, 11),
            (INPUT, 1566),
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
            (SAMPLE1, 19),
            (SAMPLE2, 23),
            (SAMPLE3, 23),
            (SAMPLE4, 29),
            (SAMPLE5, 26),
            (INPUT, 2265),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
