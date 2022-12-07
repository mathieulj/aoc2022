use anyhow::Context;
use itertools::Itertools;

fn find_marker<const SIZE: usize>(input: &str) -> Option<usize> {
    let mut buffer = ['a'; SIZE];

    for (index, char) in input.chars().enumerate() {
        buffer[index % SIZE] = char;

        if index < SIZE - 1 {
            continue;
        }

        if buffer.iter().all_unique() {
            return Some(index + 1);
        }
    }
    None
}

pub fn challenge1(input: &str) -> anyhow::Result<usize> {
    find_marker::<4>(input).context("No marker found")
}

pub fn challenge2(input: &str) -> anyhow::Result<usize> {
    find_marker::<14>(input).context("No marker found")
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
