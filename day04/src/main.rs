use anyhow::{anyhow, Result};
use itertools::{process_results, Itertools};
use std::ops::RangeInclusive;
use std::str::FromStr;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day04/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

struct Pair {
    left: Sections,
    right: Sections,
}

struct Sections(RangeInclusive<u32>);

/// A pair of elves
impl Pair {
    /// Create a new pair of elves
    fn new(line: &str) -> Result<Pair> {
        let (left, right) = process_results(line.splitn(2, ',').map(Sections::new), |ranges| {
            ranges
                .collect_tuple()
                .ok_or_else(|| anyhow!("Invalid line {}", line))
        })??;
        Ok(Pair { left, right })
    }

    fn overlaps_fully(&self) -> bool {
        (self.left.0.contains(self.right.0.start()) && self.left.0.contains(self.right.0.end()))
            || (self.right.0.contains(self.left.0.start())
                && self.right.0.contains(self.left.0.end()))
    }

    fn overlaps_partially(&self) -> bool {
        self.left.0.contains(self.right.0.start())
            || self.left.0.contains(self.right.0.end())
            || self.right.0.contains(self.left.0.start())
            || self.right.0.contains(self.left.0.end())
    }
}

impl Sections {
    fn new(range: &str) -> Result<Sections> {
        let (start, end) = process_results(range.splitn(2, '-').map(u32::from_str), |limits| {
            limits
                .collect_tuple()
                .ok_or_else(|| anyhow!("Invalid range {}", range))
        })??;
        Ok(Sections(RangeInclusive::new(start, end)))
    }
}

fn count_overlaps(input: &Input, overlapping_predicate: impl Fn(&Pair) -> bool) -> Result<u32> {
    input
        .as_lines()
        .filter(|line| !line.is_empty())
        .try_fold(0u32, |total, line| {
            Pair::new(line)
                .map(|pair| overlapping_predicate(&pair))
                .map(|overlaps| if overlaps { total + 1 } else { total })
        })
}

fn part1(input: &Input) -> Result<u32> {
    count_overlaps(input, Pair::overlaps_fully)
}

fn part2(input: &Input) -> Result<u32> {
    count_overlaps(input, Pair::overlaps_partially)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ]);
        assert_eq!(part1(&input).unwrap(), 2);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ]);
        assert_eq!(part2(&input).unwrap(), 4);
        Ok(())
    }
}
