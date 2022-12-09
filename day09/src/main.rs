use anyhow::{anyhow, bail, Result};
use itertools::{process_results, Itertools};
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use util::Input;

fn main() -> Result<()> {
    let input = Input::load("day09/input")?;

    println!("Part 1:");
    println!("{}", part1(&input)?);

    println!("Part 2:");
    println!("{}", part2(&input)?);
    Ok(())
}

/// A segment of rope, with a position in 2D-space
#[derive(Copy, Clone, Hash, Eq, PartialEq, Default, Debug)]
struct RopeSegment(i32, i32);

/// A direction of movement in 2D-space
#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Convert a string to a series of directional movements
    fn directions(instruction: &str) -> Result<impl Iterator<Item = Direction>> {
        let (direction, steps) = instruction
            .splitn(2, ' ')
            .collect_tuple()
            .ok_or_else(|| anyhow!("Invalid instruction {}", instruction))?;
        let steps = usize::from_str(steps)?;
        match direction {
            "U" => Ok(std::iter::repeat(Direction::Up).take(steps)),
            "D" => Ok(std::iter::repeat(Direction::Down).take(steps)),
            "L" => Ok(std::iter::repeat(Direction::Left).take(steps)),
            "R" => Ok(std::iter::repeat(Direction::Right).take(steps)),
            _ => bail!("Invalid instruction {}", instruction),
        }
    }
}

impl RopeSegment {
    /// Step one step in a direction
    fn step(&self, direction: Direction) -> RopeSegment {
        match direction {
            Direction::Up => RopeSegment(self.0, self.1 - 1),
            Direction::Down => RopeSegment(self.0, self.1 + 1),
            Direction::Left => RopeSegment(self.0 - 1, self.1),
            Direction::Right => RopeSegment(self.0 + 1, self.1),
        }
    }

    /// Step after another rope segment
    /// If the other segment is adjacent to this segment nothing happens
    /// If the other segment is more than 1 step away from this segment
    /// step towards it, stepping diagonally if the rope segment is
    /// neither on the same X or Y coordinate as this segment.
    fn step_after(&self, other: RopeSegment) -> RopeSegment {
        let x_distance = other.0 - self.0;
        let y_distance = other.1 - self.1;
        if x_distance.abs() <= 1 && y_distance.abs() <= 1 {
            // No movement needed
            *self
        } else if x_distance.abs() == 2 || y_distance.abs() == 2 {
            let x_step = x_distance.signum();
            let y_step = y_distance.signum();
            RopeSegment(self.0 + x_step, self.1 + y_step)
        } else {
            let x_step = x_distance.signum() * (x_distance.abs() - 1);
            let y_step = y_distance.signum() * (y_distance.abs() - 1);
            RopeSegment(self.0 + x_step, self.1 + y_step)
        }
    }
}

/// Move a rope according to a series of directions and return the number of different positions
/// the rope's tail has visited
fn move_rope(input: &Input, rope_length: usize) -> Result<u32> {
    Ok(process_results(
        input
            .trim_trailing_newlines()
            .as_lines()
            .map(Direction::directions),
        |instructions| {
            instructions.flatten().fold(
                (
                    std::iter::repeat(RopeSegment::default())
                        .take(rope_length)
                        .collect(), // Rope
                    [RopeSegment::default()].into_iter().collect::<HashSet<_>>(), // Positions visited by the tail
                ),
                |state, direction| {
                    let (rope, mut tail_positions): (VecDeque<_>, _) = state;
                    let rope = rope
                        .into_iter()
                        .fold(VecDeque::new(), |mut new_rope, segment| {
                            if new_rope.is_empty() {
                                // This is the head, step according to directions
                                new_rope.push_back(segment.step(direction));
                            } else {
                                // Step after the previous segment
                                new_rope
                                    .push_back(segment.step_after(new_rope[new_rope.len() - 1]));
                            }
                            new_rope
                        });
                    // Record the tail's position
                    tail_positions.insert(rope[rope.len() - 1]);
                    (rope, tail_positions)
                },
            )
        },
    )?
    .1
    .len() as u32)
}

fn part1(input: &Input) -> Result<u32> {
    move_rope(input, 2)
}

fn part2(input: &Input) -> Result<u32> {
    move_rope(input, 10)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};
    use anyhow::Result;
    use util::Input;

    #[test]
    pub fn test_part1() -> Result<()> {
        let input = Input::from_lines(["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"]);
        assert_eq!(part1(&input).unwrap(), 13);
        Ok(())
    }

    #[test]
    pub fn test_part2() -> Result<()> {
        let input = Input::from_lines(["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"]);
        assert_eq!(part2(&input).unwrap(), 1);
        Ok(())
    }
}
