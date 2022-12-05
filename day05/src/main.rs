use anyhow::{anyhow, Result};
use itertools::Itertools;
use once_cell::sync::OnceCell;
use regex::Regex;
use std::collections::VecDeque;
use std::str::FromStr;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day05/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

fn transpose<T: Clone>(rows: VecDeque<VecDeque<Option<T>>>) -> VecDeque<VecDeque<T>> {
    let num_cols = rows.iter().map(|row| row.len()).max().unwrap_or(0);
    (0..num_cols)
        .map(|col| {
            rows.iter()
                .filter_map(|inner| inner.get(col).cloned().flatten())
                .collect()
        })
        .collect()
}

static MOVE_REGEXP: OnceCell<Regex> = OnceCell::new();

fn move_regexp() -> &'static Regex {
    MOVE_REGEXP.get_or_init(|| Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap())
}

struct Stacks {
    stacks: VecDeque<VecDeque<char>>,
}

impl Stacks {
    fn new(layout: &str) -> Stacks {
        let rows = layout
            .split('\n')
            .map(|line| {
                line.chars()
                    .chunks(4)
                    .into_iter()
                    .map(|mut c| match c.next() {
                        Some('[') => c.next(),
                        _ => None,
                    })
                    .collect()
            })
            .collect();
        let stacks = transpose(rows);
        Stacks { stacks }
    }
}

fn operate_crane(
    input: &Input,
    move_crates: fn(count: usize, from: usize, to: usize, stacks: &mut Stacks),
) -> Result<String> {
    let (stacks, moves) = input
        .as_str()
        .splitn(2, "\n\n")
        .collect_tuple()
        .ok_or_else(|| anyhow!("Could not split into stacks and moves"))?;
    let mut stacks = Stacks::new(stacks);
    for m in moves.split('\n') {
        if let Some(captures) = move_regexp().captures(m) {
            // Lots of unwrapping going on here, but we know everything will be good because of the regex match
            let (count, from, to) = captures
                .iter()
                .skip(1)
                .map(|group| usize::from_str(group.unwrap().as_str()).unwrap())
                .collect_tuple()
                .unwrap();
            move_crates(count, from, to, &mut stacks);
        }
    }
    Ok(stacks
        .stacks
        .iter()
        .filter_map(|column| column.iter().next())
        .collect())
}

fn part1(input: &Input) -> Result<String> {
    operate_crane(input, |count, from, to, stacks| {
        for _ in 0..count {
            if let Some(c) = stacks.stacks[from - 1].pop_front() {
                stacks.stacks[to - 1].insert(0, c);
            }
        }
    })
}

fn part2(input: &Input) -> Result<String> {
    operate_crane(input, |count, from, to, stacks| {
        for i in 0..count {
            if let Some(c) = stacks.stacks[from - 1].pop_front() {
                stacks.stacks[to - 1].insert(i, c);
            }
        }
    })
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]);
        assert_eq!(part1(&input).unwrap(), "CMZ".to_string());
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]);
        assert_eq!(part2(&input).unwrap(), "MCD".to_string());
        Ok(())
    }
}
