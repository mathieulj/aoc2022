use anyhow::Context;
use itertools::Itertools;

type Range = (i64, i64);

fn range(input: &str) -> anyhow::Result<Range> {
    let (min, max) = input.split_once('-').context("Delimiter not found")?;

    Ok((min.parse()?, max.parse()?))
}

fn pair(input: &str) -> anyhow::Result<(Range, Range)> {
    let (a, b) = input.split_once(',').context("Delimiter not found")?;

    Ok((range(a)?, range(b)?))
}

fn parse(input: &str) -> impl '_ + Iterator<Item = anyhow::Result<(Range, Range)>> {
    input.lines().map(pair)
}

pub fn challenge1(input: &str) -> anyhow::Result<i64> {
    parse(input).fold_ok(0, |acc, (a, b)| {
        if a.0 <= b.0 && a.1 >= b.1 || b.0 <= a.0 && b.1 >= a.1 {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn challenge2(input: &str) -> anyhow::Result<i64> {
    parse(input).fold_ok(0, |acc, (a, b)| {
        if a.0 >= b.0 && a.0 <= b.1
            || a.1 >= b.0 && a.1 <= b.1
            || b.0 >= a.0 && b.0 <= a.1
            || b.1 >= a.0 && b.1 <= a.1
        {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");
    const SAMPLE: &str = indoc::indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (SAMPLE, 2),
            (INPUT, 424),
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
            (SAMPLE, 4),
            (INPUT, 804),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
