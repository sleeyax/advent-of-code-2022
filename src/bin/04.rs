fn parse_group(group: &str) -> (u32, u32) {
    group
        .split_once("-")
        .map(|v| (v.0.parse().unwrap(), v.1.parse().unwrap()))
        .unwrap()
}

fn is_overlapping(x: (u32, u32), y: (u32, u32)) -> bool {
    x.0.max(y.0) <= x.1.min(y.1)
}

fn is_contained(x: (u32, u32), y: (u32, u32)) -> bool {
    (y.0 >= x.0 && y.1 <= x.1) || (x.0 >= y.0 && x.1 <= y.1)
}

fn solve(input: &str, part: u32) -> Option<u32> {
    let f = if part == 1 {
        is_contained
    } else {
        is_overlapping
    };

    Some(
        input
            .lines()
            .filter(|line| {
                let (left, right) = line
                    .split_once(",")
                    .map(|(left, right)| (parse_group(left), parse_group(right)))
                    .unwrap();
                f(left, right)
            })
            .count() as u32,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
