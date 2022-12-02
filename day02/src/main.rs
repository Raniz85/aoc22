use anyhow::{anyhow, Result};
use itertools::process_results;
use once_cell::sync::OnceCell;
use std::collections::HashMap;

use util::Input;

static SCORE_TABLE: OnceCell<HashMap<&'static str, u32>> = OnceCell::new();

#[allow(clippy::identity_op)]
fn get_score_table() -> &'static HashMap<&'static str, u32> {
    SCORE_TABLE.get_or_init(|| {
        vec![
            ("A X", 1 + 3),
            ("A Y", 2 + 6),
            ("A Z", 3 + 0),
            ("B X", 1 + 0),
            ("B Y", 2 + 3),
            ("B Z", 3 + 6),
            ("C X", 1 + 6),
            ("C Y", 2 + 0),
            ("C Z", 3 + 3),
        ]
        .into_iter()
        .collect()
    })
}

static SCORE_TABLE2: OnceCell<HashMap<&'static str, u32>> = OnceCell::new();

#[allow(clippy::identity_op)]
fn get_score_table2() -> &'static HashMap<&'static str, u32> {
    SCORE_TABLE2.get_or_init(|| {
        vec![
            ("A X", 3 + 0),
            ("A Y", 1 + 3),
            ("A Z", 2 + 6),
            ("B X", 1 + 0),
            ("B Y", 2 + 3),
            ("B Z", 3 + 6),
            ("C X", 2 + 0),
            ("C Y", 3 + 3),
            ("C Z", 1 + 6),
        ]
        .into_iter()
        .collect()
    })
}

fn main() -> Result<()> {
    let input = Input::load("day02/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

fn calculate_score(input: &Input, score_table: &HashMap<&str, u32>) -> Result<u32> {
    process_results(
        input
            .as_lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                score_table
                    .get(line)
                    .ok_or_else(|| anyhow!("Invalid state {}", line))
            }),
        |scores| scores.sum(),
    )
}

fn part1(input: &Input) -> Result<u32> {
    calculate_score(input, get_score_table())
}

fn part2(input: &Input) -> Result<u32> {
    calculate_score(input, get_score_table2())
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines(["A Y", "B X", "C Z"]);
        assert_eq!(part1(&input).unwrap(), 15);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines(["A Y", "B X", "C Z"]);
        assert_eq!(part2(&input).unwrap(), 12);
        Ok(())
    }
}
