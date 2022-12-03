pub fn challenge1(input: &str) -> anyhow::Result<i64> {
    input.lines().try_fold(0, |acc, line| {
        // A: Rock
        // B: Paper
        // C: Scissors
        // X: Rock = 1
        // Y: Paper = 2
        // Z: Scissors = 3
        let score = match line.trim() {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2,
            "C Z" => 3 + 3,
            _ => anyhow::bail!("Unexpected string"),
        };

        Ok(acc + score)
    })
}

pub fn challenge2(input: &str) -> anyhow::Result<i64> {
    input.lines().try_fold(0, |acc, line| {
        // A: Rock
        // B: Paper
        // C: Scissors
        // X: Loss = 0
        // Y: Draw = 3
        // Z: Win = 6
        let score = match line.trim() {
            "A X" => 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,
            "B X" => 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 2,
            "C Y" => 3 + 3,
            "C Z" => 6 + 1,
            _ => anyhow::bail!("Unexpected string"),
        };

        Ok(acc + score)
    })
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../input.txt");
    const SAMPLE: &str = indoc::indoc! {"
        A Y
        B X
        C Z"
    };

    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (SAMPLE, 15),
            (INPUT, 14163),
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
            (SAMPLE, 12),
            (INPUT, 12091),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
