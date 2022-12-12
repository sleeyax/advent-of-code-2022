use derive_more::{Add, AddAssign, Mul, Sub};
use std::collections::HashSet;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("unknown direction {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Add, AddAssign, Mul, Sub)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Returns the maximum value of `x` and `y`.
    fn max(&self) -> i32 {
        self.x.max(self.y)
    }

    /// Returns a new instance of self where each `number` represents the sign of itself.
    ///   * 0 if the number is zero
    ///   * 1 if the number is positive
    ///   * -1 if the number is negative
    ///
    /// Where `number` can be any of its fields (`x` or `y`).
    fn signum(&self) -> Self {
        Self::new(self.x.signum(), self.y.signum())
    }

    /// Returns a new instance of self with all fields set to their absolute values.
    fn abs(&self) -> Self {
        Self::new(self.x.abs(),  self.y.abs())
    }
}

impl From<&Direction> for Point {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::Right => Self::new(1, 0),
            Direction::Left => Self::new(-1, 0),
            Direction::Up => Self::new(0, 1),
            Direction::Down => Self::new(0, -1),
        }
    }
}

fn solve(input: &str, length: usize) -> u32 {
    // we start our knotting yourney at the bottom left of our 2D grid (x=0, y=0).
    let start_point = Point::new(0, 0);

    // we also have `length` knots as part of the rope (first part: 1 head + 1 tail)
    // we store their curent positions in this array.
    let mut knots = vec![start_point; length];

    // an array containing all `Point`s the tail has been.
    // we specifically use a hashset so no duplicates are inserted.
    let mut tail_points = HashSet::new();
    tail_points.insert(start_point);

    for line in input.lines() {
        let (direction, count) = line
            .split_once(" ")
            .and_then(|(pos, n)| Some((Direction::from(pos), n.parse::<usize>().unwrap())))
            .unwrap();

        for _ in 0..count {
            // move the head `count` amount of times in the `direction`
            knots[0] += Point::from(&direction);

            // move the rest of the knots `count` amount of times in the `direction`
            for i in 1..knots.len() {
                // diff between previous knot and current knot
                let diff = knots[i - 1] - knots[i];

                // if the head is already right next to the tail, continue
                if diff.abs().max() <= 1 {
                    continue;
                }

                // update current knot's position
                knots[i] += diff.signum();
            }

            tail_points.insert(*knots.last().unwrap());
        }
    }

    tail_points.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, 10))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
