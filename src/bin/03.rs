use std::collections::{HashMap, HashSet};

fn to_priority(ch: char) -> u32 {
    let offset: u32 = if ch.is_uppercase() { 38 } else { 0 };
    let ord: u32 = ch.into();
    (ord % 96) - offset
}

/// Splits the given string into multiple compartements.
/// Currently only supports max. 2 compartements!
fn split_into_compartements(line: &str, count: usize) -> Vec<String> {
    assert!(count <= 2);
    let mid = line.len() / count;
    let (left, right) = line.split_at(mid);
    vec![left.into(), right.into()]
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;

    for line in input.lines() {
        let compartements = split_into_compartements(line, 2);

        // Dedupe each compartements' items and merge them back to a single string so we can easily identify a duplicate.
        // Please note that this code isn't very extendable by design, I wanted to have a more direct solution to solve part 1.
        // E.g. `["aabb", "add"]` -> `["ab", "ad"]` -> `"abad"` -> duplicate found is "a"
        let deduped = compartements
            .iter()
            .map(|compartement| {
                compartement
                    .chars()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect::<String>()
            })
            .collect::<String>();

        let mut letters = HashMap::new();

        for ch in deduped.chars() {
            if letters.contains_key(&ch) {
                let priority = to_priority(ch);
                total += priority;
            }
            letters.entry(ch).or_insert(1);
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
