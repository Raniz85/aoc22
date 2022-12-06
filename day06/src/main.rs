use anyhow::{anyhow, Result};
use itertools::Itertools;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day06/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

fn find_marker(input: &Input, marker_size: usize) -> Result<usize> {
    input
        .as_str()
        .chars()
        .collect_vec()
        .windows(marker_size)
        .enumerate()
        .find(|(_, window)| window.iter().unique().count() == marker_size)
        .map(|(index, _)| index + marker_size)
        .ok_or_else(|| anyhow!("No marker in input"))
}

fn part1(input: &Input) -> Result<usize> {
    find_marker(input, 4)
}

fn part2(input: &Input) -> Result<usize> {
    find_marker(input, 14)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        for (input, expected_index) in [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ] {
            let input = Input::from_lines([input]);
            assert_eq!(part1(&input).unwrap(), expected_index);
        }
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        for (input, expected_index) in [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ] {
            let input = Input::from_lines([input]);
            assert_eq!(part2(&input).unwrap(), expected_index);
        }
        Ok(())
    }
}
