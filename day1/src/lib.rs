pub fn challenge1(input: &str) -> anyhow::Result<i64> {
    let (acc, max) = input.lines().try_fold((0, 0), |(acc, max), line| {
        if line.is_empty() {
            Ok::<_, anyhow::Error>((0, acc.max(max)))
        } else {
            let calories: i64 = line.parse()?;
            Ok((acc + calories, max))
        }
    })?;

    Ok(acc.max(max))
}

pub fn challenge2(input: &str) -> anyhow::Result<i64> {
    let mut total = input.lines().try_fold(vec![0], |mut acc, line| {
        if line.is_empty() {
            acc.push(0);
        } else {
            let calories: i64 = line.parse()?;
            *acc.last_mut().unwrap() += calories;
        }
        Ok::<_, anyhow::Error>(acc)
    })?;

    total.sort();

    Ok(total.into_iter().rev().take(3).sum())
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
    #[test]
    fn test_challenge1() -> anyhow::Result<()> {
        let expected = [
            // Add tests
            (EXAMPLE, 24000),
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
            (EXAMPLE, 45000),
        ];
        for (input, output) in expected {
            assert_eq!(crate::challenge2(input)?, output, "For input {}", input)
        }
        Ok(())
    }
}
