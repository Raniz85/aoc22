use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use std::collections::HashSet;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day03/input")?;

    println!("Part 1:");
    println!("{}", part1(&input));

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

/// A piece of gear in an elfs backpack
#[derive(Hash, PartialOrd, PartialEq, Ord, Eq, Copy, Clone)]
struct Gear(char);

impl Gear {
    /// Get the priority of this piece of gear
    fn priority(&self) -> u32 {
        if 'a' <= self.0 && self.0 <= 'z' {
            self.0 as u32 - 'a' as u32 + 1
        } else if 'A' <= self.0 && self.0 <= 'Z' {
            self.0 as u32 - 'A' as u32 + 27
        } else {
            0
        }
    }
}

fn part1(input: &Input) -> u32 {
    input
        .trim_trailing_newlines()
        .as_lines()
        .flat_map(|line| {
            // Split the line in the middle to get the two compartments and convert them into hashsets of gear
            let (first, second) = line.split_at(line.len() / 2);
            let first: HashSet<Gear> = first.chars().map(Gear).collect();
            let second: HashSet<Gear> = second.chars().map(Gear).collect();
            // Find the duplicates by using set intersection
            let duplicates = first.intersection(&second);
            // convert into priority
            duplicates.into_iter().map(Gear::priority).collect_vec()
        })
        .sum()
}

fn part2(input: &Input) -> Result<u32> {
    Ok(input
        .trim_trailing_newlines()
        .as_lines()
        .collect_vec()
        .as_slice()
        .chunks(3)
        .map(|group| {
            // Iterate over sets of 3 elves and convert their backpacks into hashsets of gear
            let elves: [HashSet<Gear>; 3] = group
                .iter()
                .map(|elf| elf.chars().map(Gear).collect::<HashSet<Gear>>())
                .collect_vec()
                .try_into()
                .map_err(|_| anyhow!("Number of elves not divisible by 3"))?;
            // Find the element in all three backpacks using set intersection
            let badge = elves[0]
                .intersection(&elves[1])
                .cloned()
                .collect::<HashSet<Gear>>()
                .intersection(&elves[2])
                .cloned()
                .collect_vec();
            if badge.len() != 1 {
                bail!("Invalid group with {} shared items", badge.len())
            } else {
                Ok(badge.into_iter().next().unwrap())
            }
        })
        .try_collect::<_, Vec<_>, _>()?
        .iter()
        .map(Gear::priority)
        .sum())
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]);
        assert_eq!(part1(&input), 157);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]);
        assert_eq!(part2(&input).unwrap(), 70);
        Ok(())
    }
}
