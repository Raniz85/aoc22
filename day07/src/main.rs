use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day07/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Path<'a> {
    segments: Vec<&'a str>,
}

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}/", self.segments.iter().join("/"))
    }
}

/// A path consisting of directories
impl<'a> Path<'a> {
    fn empty() -> Path<'a> {
        Path { segments: vec![] }
    }

    /// Append a directory to this path, descending into it
    fn descend(self, segment: &'a str) -> Path<'a> {
        Path {
            segments: self
                .segments
                .into_iter()
                .chain([segment].into_iter())
                .collect_vec(),
        }
    }

    /// Ascend a directory, dropping the last directory of this path
    fn ascend(self) -> Path<'a> {
        let len = 0.max(self.segments.len() - 1);
        Path {
            segments: self.segments.into_iter().take(len).collect_vec(),
        }
    }

    /// Get all paths leading up to this path.
    ///
    /// I.E. /a/b/c will yield
    /// /
    /// /a
    /// /a/b
    /// /a/b/c
    fn get_path_hierarchy(&self) -> Vec<String> {
        let mut paths = Vec::new();
        let mut path = "/".to_string();
        for segment in &self.segments {
            paths.push(path.clone());
            path.push_str(segment);
            path.push('/');
        }
        paths.push(path.clone());
        paths
    }
}

/// Parse a directory traversal log, recording file and directory sizes on the way
///
/// returns a map of all directories and the total size of the files and directories contained within
fn get_directories(input: &Input) -> Result<HashMap<String, u32>> {
    let directories = input
        .as_lines()
        .try_fold(
            (HashMap::new(), Path::empty()),
            |state, line| -> Result<_> {
                let (mut directories, path) = state;
                //  interpret the line
                let path = match line {
                    // a 'cd' command
                    cd if line.starts_with("$ cd") => match &cd[5..] {
                        ".." => path.ascend(),
                        "/" => Path::empty(),
                        subdir => path.descend(subdir),
                    },
                    // a file size
                    file if line.chars().next().map(|c| c.is_numeric()) == Some(true) => {
                        // split into file size and file name and add the size to this and all parent directories
                        let (size, _name) = file
                            .splitn(2, ' ')
                            .map(str::trim)
                            .collect_tuple()
                            .ok_or_else(|| anyhow!("Invalid file description {}", file))?;
                        let size = u32::from_str(size)?;
                        for parent in path.get_path_hierarchy() {
                            directories
                                .entry(parent)
                                .and_modify(|total| *total += size)
                                .or_insert(size);
                        }
                        path
                    }
                    // ls command or a directory listing
                    _ => path,
                };
                Ok((directories, path))
            },
        )?
        .0;
    Ok(directories)
}

fn part1(input: &Input) -> Result<u32> {
    let directories = get_directories(input)?;
    Ok(directories.values().filter(|size| **size <= 100000).sum())
}

fn part2(input: &Input) -> Result<u32> {
    let directories = get_directories(input)?;
    let free_space = 70000000 - directories.get("/").cloned().unwrap_or(0);
    let space_to_free = 30000000u32.saturating_sub(free_space);
    directories
        .values()
        .filter(|size| **size >= space_to_free)
        .min()
        .cloned()
        .ok_or_else(|| anyhow!("No matching directory found"))
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines([
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ]);
        assert_eq!(part1(&input).unwrap(), 95437);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines([
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ]);
        assert_eq!(part2(&input).unwrap(), 24933642);
        Ok(())
    }
}
