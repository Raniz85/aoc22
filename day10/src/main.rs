use anyhow::{bail, Result};
use itertools::Itertools;
use std::str::FromStr;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day10/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

fn run_computation(input: &Input) -> Result<Vec<i32>> {
    let timeline = input
        .trim_trailing_newlines()
        .as_lines()
        .try_fold(
            (1, Vec::new()),
            |mut state, instruction| match &instruction[0..4] {
                "noop" => {
                    state.1.push(state.0);
                    Ok(state)
                }
                "addx" => {
                    let amount = i32::from_str(&instruction[5..])?;
                    state.1.push(state.0);
                    state.1.push(state.0);
                    Ok((state.0 + amount, state.1))
                }
                _ => bail!("Invalid instruction {}", instruction),
            },
        )?
        .1;
    Ok(timeline)
}

fn part1(input: &Input) -> Result<i32> {
    let timeline = run_computation(input)?;
    Ok(timeline
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(index, value)| (index + 1) as i32 * value)
        .sum())
}

fn part2(input: &Input) -> Result<String> {
    let timeline = run_computation(input)?;
    Ok(timeline
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let pixel = (index % 40) as i32;
            if pixel >= value - 1 && pixel <= value + 1 {
                "#"
            } else {
                " "
            }
        })
        .chunks(40)
        .into_iter()
        .map(|mut chunk| chunk.join(""))
        .join("\n")
        + "\n")
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::load("test_input")?;
        assert_eq!(part1(&input).unwrap(), 13140);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::load("test_input")?;
        let expected = "\
            ##..##..##..##..##..##..##..##..##..##..\n\
            ###...###...###...###...###...###...###.\n\
            ####....####....####....####....####....\n\
            #####.....#####.....#####.....#####.....\n\
            ######......######......######......####\n\
            #######.......#######.......#######.....\n";
        assert_eq!(&part2(&input).unwrap(), expected);
        Ok(())
    }
}
