// I really dislike implementing existing pathfinfing algos from scratch (what's fun about reinventing the wheel??)
// so I heavily based this code on https://github.com/ericwburden/advent_of_code_2022/blob/main/src/day12/input.rs
// which implements Dijkstra's algorithm (https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Algorithm).
// I'm hoping day 13 will be more fun...

use std::{collections::{HashMap, BinaryHeap}, cmp::Reverse};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Mountain {
    height: u8,
    _type: MountainType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MountainType {
    Start,
    End,
    Mountain,
}

impl From<char> for Mountain {
    fn from(value: char) -> Self {
        match value {
            'S' => Mountain { height: 0, _type: MountainType::Start },
            'E' => Mountain { height: 25, _type: MountainType::End },
            c if c.is_ascii_lowercase() => Mountain { height: value as u8 - b'a', _type: MountainType::Mountain },
            _ => unreachable!(),
        }
    }
}

impl Mountain {
    fn can_reach(&self, other: &Mountain) -> bool {
        other.height.saturating_sub(self.height) <= 1
    }
}

type Neighbors = [Option<(usize, usize)>; 4];

struct MountainMap {
    mountains: Vec<Vec<Mountain>>,
    graph: HashMap<(usize, usize), Neighbors>,
    start_at: (usize, usize),
    end_at: (usize, usize),
}

impl From<&str> for MountainMap {
    fn from(value: &str) -> Self {
        let mountains: Vec<Vec<_>> = value
            .lines()
            .map(|row| row.chars().map(Mountain::from).collect())
            .collect();

        let mut graph = HashMap::new();

        let last_row = mountains.len().saturating_sub(1);
        let last_col = mountains
            .first()
            .map(|r| r.len())
            .unwrap_or_default()
            .saturating_sub(1);

        let mut start_at = (0, 0);
        let mut end_at = (0, 0);

        for (row_idx, row) in mountains.iter().enumerate() {
            for (col_idx, mountain) in row.iter().enumerate() {
                // Create and fill in the array of neighbors in order of direction
                // from up, left, down, and right. We're doing our
                // bounds checking here as well, just to save on possible edge cases
                // later on.
                let mut neighbors = [None; 4];
                if row_idx > 0 && mountain.can_reach(&mountains[row_idx - 1][col_idx]) {
                    neighbors[0] = Some((row_idx - 1, col_idx));
                }
                if col_idx > 0 && mountain.can_reach(&mountains[row_idx][col_idx - 1]) {
                    neighbors[1] = Some((row_idx, col_idx - 1));
                }
                if row_idx < last_row && mountain.can_reach(&mountains[row_idx + 1][col_idx]) {
                    neighbors[2] = Some((row_idx + 1, col_idx));
                }
                if col_idx < last_col && mountain.can_reach(&mountains[row_idx][col_idx + 1]) {
                    neighbors[3] = Some((row_idx, col_idx + 1));
                }

                // When we encounter the start or end mountains, we mark those as special.
                match mountain._type {
                    MountainType::Start => start_at = (row_idx, col_idx),
                    MountainType::End => end_at = (row_idx, col_idx),
                    _ => {},
                }

                graph.insert((row_idx, col_idx), neighbors);
            }
        }

        MountainMap {
            mountains,
            graph,
            start_at,
            end_at,
        }
    }
}

impl MountainMap {
    /// Implement Dijkstra's Algorithm
    fn shortest_path_to_summit(&self, start_at: (usize, usize)) -> Option<u32> {
        // The 'open set': the mountains we know how to travel to, but don't
        // know how to travel _from_ yet. You can think of this like the expanding
        // outer edge of our search space, if that helps. Because it's a binary
        // heap (we're using as a _min_ binary heap), the next mountain to be fetched
        // will always be the one with the shortest travel time found so far.
        let mut open = BinaryHeap::from([(Reverse(0), start_at)]);

        // Maintain a listing of the shortest number of steps to each mountain we've
        // traveled to. It's the shortest number of steps _so far_, it's possible
        // to update these if a shorter path is found.
        let mut steps = HashMap::from([(start_at, 0)]);

        // So long as there are mountains left to climb...
        while let Some((_, pos)) = open.pop() {
            // Check the mountain we're currently on. If it's the end, then just return
            // the number of steps it took us to get here.
            let (row, col) = pos;
            if pos == self.end_at {
                return steps.get(&pos).copied();
            }

            // Otherwise, see if this mountain has any neighbors we can reach. If not,
            // skip it and move on to the next mountain in our 'open set'.
            let neighbors = match self.graph.get(&pos) {
                Some(neighbors) => neighbors,
                _ => { continue; }
            };

            // For each direction where there might be a neighbor...
            for maybe_neighbor in neighbors {
                // If there's no reachable neighbor, try the next direction.
                let neighbor = match maybe_neighbor {
                    Some(neighbor) => neighbor,
                    _ => { continue; }
                };

                // Otherwise, calculate how many steps it will take to get to that
                // neighbor from the path you're currently on. That is, one more step
                // than it took to get to the current mountain.
                let next_steps: u32 = steps.get(&pos).unwrap() + 1;

                // Check how many steps are in the current shortest path to that neighbor
                let curr_steps: u32 = *steps.get(neighbor).unwrap_or(&u32::MAX);

                // If we've already found a shorter way to get there, we can just
                // move on.
                if next_steps >= curr_steps {
                    continue;
                }

                // If we're on the shortest path, then add the neighbor to the open
                // set and record the number of steps
                open.push((Reverse(next_steps), *neighbor));
                steps.insert(*neighbor, next_steps);
            }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = &MountainMap::from(input);
    map.shortest_path_to_summit(map.start_at).unwrap().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
