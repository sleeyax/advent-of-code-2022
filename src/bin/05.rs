use std::collections::{BTreeMap, VecDeque, btree_map::Entry};

// NOTE: Parsing the header took me way more time than parsing the steps and solving the actual challenge.
// I started by hard-coding the crates and solved the actual challenge first.
// Then I got to parsing the header but it took too much time so I ended up taking some inspiration from https://github.com/believer/advent-of-code/blob/master/rust/2022/src/day_05.rs which has a similar approach I had in mind.

struct Step {
    to: usize,
    from: usize,
    count: usize,
}

type Stacks = BTreeMap<usize, VecDeque<String>>;
type Steps = Vec<Step>;

impl Step {
    fn new(input: &str) -> Step {
        let parts: Vec<&str> = input.split_whitespace().collect();

        Step {
            to: parts[5].parse::<usize>().unwrap() - 1,
            from: parts[3].parse::<usize>().unwrap() - 1,
            count: parts[1].parse().unwrap(),
        }
    }
}

fn parse_input(input: &str) -> (Stacks, Steps) {
    let mut map: Stacks = BTreeMap::new();

    let (crates, steps) = input.split_once("\n\n").unwrap();

    // parse the crates
    for row in crates
        .lines()
        .collect::<Vec<&str>>()
        .split_last() // we don't need the last line with the numbers
        .unwrap()
        .1
    {
        for (i, column) in row.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            // find the name of the crate
            let value: String = column
                .iter()
                .map(|s| s.to_string().trim().replace(['[', ']'], ""))
                .collect::<Vec<String>>()
                .join("");

            // skip empty columns
            if value.is_empty() {
                continue;
            }

            // add to or create the stack
            match map.entry(i) {
                Entry::Vacant(e) => {
                    e.insert(VecDeque::from_iter([value]));
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push_back(value);
                }
            }
        }
    }

    // parse the instructions
    let instructions: Vec<Step> = steps
        .lines()
        .map(Step::new)
        .collect();

    (map, instructions)
}

fn find_first_crates(stacks: Stacks) -> String {
    stacks
        .values()
        .map(|v| v.front().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, steps) = parse_input(input);

    for Step { count, from, to } in steps {
        for _ in 0..count {
            let crate_ = stacks.get_mut(&from).unwrap().pop_front().unwrap();
            stacks.get_mut(&to).unwrap().push_front(crate_);
        }
    }

    Some(find_first_crates(stacks))
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, steps) = parse_input(input);

    for Step { count, from, to } in steps {
        // Remove all crates that should be moved from the source stack.
        // Reverse the order since we're draining from the front.
        let stack: VecDeque<String> = stacks.get_mut(&from).unwrap().drain(..count).rev().collect();

        // Add the crates to the destination stack
        for crate_ in stack {
            stacks.get_mut(&to).unwrap().push_front(crate_);
        }
    }

    Some(find_first_crates(stacks))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".into()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".into()));
    }
}
