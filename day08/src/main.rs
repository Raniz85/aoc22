use anyhow::{anyhow, bail, Result};
use itertools::Itertools;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day08/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

fn build_forest(input: &Input) -> Result<(Vec<u32>, usize, usize)> {
    if input.as_lines()
        .filter(|line| !line.is_empty())
        .map(str::len)
        .unique()
        .count() != 1 {
        bail!("All rows are not of the same length");
    }
    let forest = input.as_lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.chars().map(|c| c as u32 - '0' as u32))
        .collect_vec();
    let num_rows = input.as_lines().filter(|line| !line.is_empty()).count();
    let num_cols = forest.len() / num_rows;
    Ok((forest, num_rows, num_cols))
}

fn count_trees(forest: &[u32], iterator: impl Iterator<Item=usize>) -> Vec<usize> {
    iterator
        .fold((-1, Vec::new()), |state, index| {
            let (max_tree, mut trees) = state;
            let tree = forest[index];
            let max_tree = if tree as i64 > max_tree {
                trees.push(index);
                tree as i64
            } else {
                max_tree
            };
            (max_tree, trees)
        }).1
}

fn part1(input: &Input) -> Result<u32> {
    let (forest, num_rows, num_cols) = build_forest(input)?;
    let rows_iterator = (0..num_rows)
        .flat_map(|row| -> [Box<dyn Iterator<Item=usize>>;2] {
            let row_start = row * num_cols;
            let row_end = (row + 1) * num_cols;
            [Box::new(row_start..row_end), Box::new((row_start..row_end).rev())]
        });
    let cols_iterator = (0..num_cols)
        .flat_map(|col| -> [Box<dyn Iterator<Item=usize>>;2] {
            let col_start = col;
            let col_end = num_rows * num_cols + col;
            [Box::new((col_start..col_end).step_by(num_cols)), Box::new((col_start..col_end).step_by(num_cols).rev())]
        });
    Ok(rows_iterator.chain(cols_iterator)
        .flat_map(|tree_iterator| count_trees(&forest, tree_iterator))
        .unique()
        .count() as u32)
}

fn visible_trees(forest: &[u32], mut it: impl Iterator<Item=usize>) -> usize {
    if let Some(tree_index) = it.next() {
        let tree = forest[tree_index];
        let mut seen = 0;
        for index in it {
            seen += 1;
            if forest[index] >= tree {
                break;
            }
        }
        seen
    } else {
        0
    }
}

fn part2(input: &Input) -> Result<u32> {
    let (forest, _num_rows, num_cols) = build_forest(input)?;
    Ok((0..forest.len())
        .map(|tree_index| {
            let row = tree_index / num_cols;
            let left_trees = visible_trees(&forest, ((row * num_cols)..=tree_index).rev());
            let right_trees = visible_trees(&forest, tree_index..((row + 1) * num_cols));
            let up_trees = visible_trees(&forest, ((tree_index % num_cols)..=tree_index).rev().step_by(num_cols));
            let down_trees = visible_trees(&forest, (tree_index..forest.len()).step_by(num_cols));
            left_trees * right_trees * up_trees * down_trees
        })
        .max()
        .ok_or_else(|| anyhow!("No trees in the forest"))? as u32)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
                                          "30373",
                                      "25512",
                                      "65332",
                                      "33549",
                                      "35390",
        ]);
        assert_eq!(part1(&input).unwrap(), 21);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "30373",
            "25512",
            "65332",
            "33549",
            "35390",
        ]);
        assert_eq!(part2(&input).unwrap(), 8);
        Ok(())
    }
}
