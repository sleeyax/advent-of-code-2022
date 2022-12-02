pub fn part_one(input: &str) -> Option<u32> {
    // I really wanted a declarative solution here.
    // The 'filter' at the end is not really necessary, but without it the resulting iterator will contain a bunch of ugly 0 values.
    input
        .split("\n")
        .into_iter()
        .scan(0 as u32, |state, item| {
            if let Some(calories) = item.parse::<u32>().ok() {
                *state = *state + calories;
                Some(0)
            } else {
                let result = Some(*state);
                *state = 0;
                result
            }
        })
        .filter(|calories| *calories != 0)
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
