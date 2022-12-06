use itertools::Itertools;

fn find_marker(line: &str, marker_length: usize) -> Option<u32> {
    for (i, _)  in line.chars().enumerate() {
        let n = i+1;
        let marker = line.chars().skip(n).take(marker_length); // this probably isn't the most efficient
        if marker.sorted().dedup().count() == marker_length {
            return Some((n + marker_length) as u32);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let line = input.lines().next().unwrap();
    find_marker(line, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let line = input.lines().next().unwrap();
    find_marker(line, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
