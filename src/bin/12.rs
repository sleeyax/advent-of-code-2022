// I really dislike implementing existing pathfinding algos from scratch (what's fun about reinventing the wheel??)
// so I heavily based this code on https://github.com/ericwburden/advent_of_code_2022/blob/main/src/day12/input.rs
// which implements Dijkstra's algorithm (https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Algorithm).
// I'm hoping day 13 will be more fun...

use std::{collections::{HashMap, BinaryHeap}, cmp::{Reverse, min}};

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

struct DescentMap {
    mountains: Vec<Vec<Mountain>>,
    graph: HashMap<(usize, usize), Neighbors>,
    summit: (usize, usize),
}

impl From<&MountainMap> for DescentMap {
    fn from(map: &MountainMap) -> Self {
        let mut graph: HashMap<(usize, usize), Neighbors> = HashMap::new();

        for (pos, neighbors) in map.graph.iter() {
            // For each neighbor in the entry's list of neighbors (skipping the empty
            // spaces in the neighbor array)
            for neighbor in neighbors.iter().flatten() {
                // We're checking the entry in our inverted `graph` where the
                // neighbor is the key, creating an entry with an empty set of
                // neighbors if the neighbor doesn't have an entry yet. Then, for each
                // slot in the value array for `neighbor`, find the first index that
                // doesn't have a value yet and put `pos` there. This 'inverts' the
                // relationships by making `neighbor` the key and adding `pos` as one
                // of the positions from which `neighbor` can be reached.
                graph
                    .entry(*neighbor)
                    .or_default()
                    .iter_mut()
                    .filter(|slot| slot.is_none())
                    .take(1)
                    .for_each(|slot| *slot = Some(*pos));
            }
        }

        let mountains = map.mountains.to_vec();
        let summit = map.end_at;

        // Return the new `DescentMap` with the inverted graph.
        DescentMap {
            mountains,
            graph,
            summit,
        }
    }
}

impl DescentMap {
    /// Identify and return the minimum number of steps every other mountain is from
    /// the summit as a HashMap where the keys are mountain positions and the values
    /// are the number of steps from the summit.
    pub fn shortest_paths_from_summit(&self) -> HashMap<(usize, usize), u32> {
        // The procedure here is the same Dijkstra's algorithm from part one, just
        // walking down from the summit instead of up from the start space.
        let start_at = self.summit;
        let mut open = BinaryHeap::from([(Reverse(0), start_at)]);
        let mut steps = HashMap::from([(start_at, 0)]);

        // While there are still mountains to explore...
        while let Some((_, pos)) = open.pop() {
            // No need for an early return here, we want to find a path to _all_ the
            // other mountains.

            // As before, we check all the neighbors and any time we're able to
            // reach that neighbor by the shortest path found so far, we add that
            // neighbor to the open set.
            let neighbors = match self.graph.get(&pos) {
                Some(neighbors) => neighbors,
                _ => { continue; }
            };
            for maybe_neighbor in neighbors {
                let neighbor = match maybe_neighbor {
                    Some(neighbor) => neighbor,
                    _ => { continue; }
                };
                let next_steps: u32 = steps.get(&pos).unwrap() + 1;
                let curr_steps: u32 = *steps.get(neighbor).unwrap_or(&u32::MAX);
                if next_steps >= curr_steps {
                    continue;
                }
                open.push((Reverse(next_steps), *neighbor));
                steps.insert(*neighbor, next_steps);
            }
        }

        // Returns a mapping of the fewest steps to every mountain from the summit
        steps
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = &MountainMap::from(input);
    map.shortest_path_to_summit(map.start_at).unwrap().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let descent_map = DescentMap::from(&MountainMap::from(input));

    let steps_to_short_mountains = descent_map.shortest_paths_from_summit();

    let mut shortest_path = u32::MAX;
    for (pos, steps_to_pos) in steps_to_short_mountains.iter() {
        let (row, col) = *pos;
        if descent_map.mountains[row][col]._type == MountainType::Mountain && descent_map.mountains[row][col].height == 0 {} else {continue;}
        shortest_path = min(shortest_path, *steps_to_pos);
    }

    shortest_path.into()
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
        assert_eq!(part_two(&input), Some(29));
    }
}
