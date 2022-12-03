use itertools::Itertools;
use std::collections::HashSet;

fn score(input: char) -> i64 {
    if input.is_lowercase() {
        input as i64 - 'a' as i64 + 1
    } else {
        input as i64 - 'A' as i64 + 27
    }
}

pub fn challenge1(input: &str) -> anyhow::Result<i64> {
    Ok(input.lines().fold(0, |acc, line| {
        let (left, right) = line.split_at(line.len() / 2);
        let left: HashSet<char> = left.chars().collect();
        let right: HashSet<char> = right.chars().collect();

        acc + left.intersection(&right).map(|c| score(*c)).sum::<i64>()
    }))
}

pub fn challenge2(input: &str) -> anyhow::Result<i64> {
    Ok(input
        .lines()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .chunks(3)
        .into_iter()
        .filter_map(|chunks| chunks.reduce(|a, b| a.intersection(&b).copied().collect()))
        .flatten()
        .map(score)
        .sum())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");
    const SAMPLE: &str = indoc::indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"
    };

    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (SAMPLE, 157),
            (INPUT, 8243),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge1(input)?, output, "For input {}", input)
        }

        assert_eq!(crate::challenge1(INPUT)?, 8243);

        Ok(())
    }

    #[test]
    fn test_challenge2() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (SAMPLE, 70),
            (INPUT, 2631),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
