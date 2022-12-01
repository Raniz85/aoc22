use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::str::FromStr;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day01/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

fn get_elves(input: &Input) -> Result<Vec<u32>> {
    Ok(input
        .as_lines()
        .batching(|it| {
            match it
                .take_while(|calories| !calories.is_empty())
                .map(u32::from_str)
                .try_collect::<_, Vec<_>, _>()
            {
                Ok(empty) if empty.is_empty() => None,
                Ok(elf) => Some(Ok(elf.into_iter().sum())),
                Err(error) => Some(Err(error)),
            }
        })
        .try_collect::<_, Vec<_>, _>()?)
}

fn part1(input: &Input) -> Result<u32> {
    get_elves(input)?
        .into_iter()
        .max()
        .ok_or_else(|| anyhow!("No calories in input"))
}
fn part2(input: &Input) -> Result<u32> {
    Ok(get_elves(input)?.into_iter().sorted().rev().take(3).sum())
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]);
        assert_eq!(part1(&input).unwrap(), 24000);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ]);
        assert_eq!(part2(&input).unwrap(), 45000);
        Ok(())
    }
}
