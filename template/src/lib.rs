pub fn challenge1(_input: &str) -> anyhow::Result<i64> {
    Ok(0)
}

pub fn challenge2(_input: &str) -> anyhow::Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    // const INPUT: &str = include_str!("../input.txt");
    const SAMPLE: &str = indoc::indoc! {"
    "};

    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (SAMPLE, 0),
            // (INPUT, 0),
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
            (SAMPLE, 0),
            // (INPUT, 0),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
