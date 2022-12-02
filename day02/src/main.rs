use anyhow::{anyhow, Result};
use itertools::process_results;
use once_cell::sync::OnceCell;
use std::collections::HashMap;

use util::Input;

static SCORE_TABLE: OnceCell<HashMap<&'static str, u32>> = OnceCell::new();

/// Score table for part 1
#[allow(clippy::identity_op)]
fn get_score_table() -> &'static HashMap<&'static str, u32> {
    SCORE_TABLE.get_or_init(|| {
        vec![
            ("A X", 1 + 3), // Rock vs rock
            ("A Y", 2 + 6), // Rock vs paper
            ("A Z", 3 + 0), // Rock vs scissors
            ("B X", 1 + 0), // Paper vs rock
            ("B Y", 2 + 3), // Paper vs paper
            ("B Z", 3 + 6), // Paper vs scissors
            ("C X", 1 + 6), // Scissors vs rock
            ("C Y", 2 + 0), // Scissors vs paper
            ("C Z", 3 + 3), // Scissors vs scissors
        ]
        .into_iter()
        .collect()
    })
}

static SCORE_TABLE2: OnceCell<HashMap<&'static str, u32>> = OnceCell::new();

/// Score table for part 2
#[allow(clippy::identity_op)]
fn get_score_table2() -> &'static HashMap<&'static str, u32> {
    SCORE_TABLE2.get_or_init(|| {
        vec![
            ("A X", 3 + 0), // Loss vs rock means scissors
            ("A Y", 1 + 3), // Draw vs rock means rock
            ("A Z", 2 + 6), // Win vs rock means paper
            ("B X", 1 + 0), // Loss vs paper means rock
            ("B Y", 2 + 3), // Draw vs paper means paper
            ("B Z", 3 + 6), // Win vs paper means scissors
            ("C X", 2 + 0), // Loss vs scissors means paper
            ("C Y", 3 + 3), // Draw vs scissors means scissors
            ("C Z", 1 + 6), // Win vs scissors means rock
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

/// Calculate the total score based on a score table
fn calculate_score(input: &Input, score_table: &HashMap<&str, u32>) -> Result<u32> {
    // Iterate over all lines, fetching the score from the score table and the summing it up
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
