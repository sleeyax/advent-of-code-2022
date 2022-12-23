use itertools::Itertools;

#[derive(Debug)]
struct Test {
    divisible_by: isize,
    if_true: isize,
    if_false: isize,
}

#[derive(Debug)]
struct Monkey {
    nr: isize,
    items: Vec<isize>,
    operation: String,
    test: Test,
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        // TODO: there's probably a better way to parse a monkey; use regex or https://github.com/Geal/nom
        let mut iter = s.lines().map(|line| line.trim());
        let nr = iter
            .next()
            .unwrap()
            .replace("Monkey ", "")
            .replace(":", "")
            .parse::<isize>()
            .unwrap();
        let starting_items = iter
            .next()
            .unwrap()
            .replace("Starting items: ", "")
            .split(",")
            .map(|v| v.trim().parse::<isize>().unwrap())
            .collect_vec();
        let operation = iter.next().unwrap().replace("Operation: new = ", "");
        let test_divisible_by = iter
            .next()
            .unwrap()
            .replace("Test: divisible by ", "")
            .parse::<isize>()
            .unwrap();
        let test_if_true = iter
            .next()
            .unwrap()
            .replace("If true: throw to monkey ", "")
            .parse::<isize>()
            .unwrap();
        let test_if_false = iter
            .next()
            .unwrap()
            .replace("If false: throw to monkey ", "")
            .parse::<isize>()
            .unwrap();

        Monkey {
            nr,
            items: starting_items,
            operation,
            test: Test {
                divisible_by: test_divisible_by,
                if_true: test_if_true,
                if_false: test_if_false,
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    for line in input.split("\n\n") {
        // TODO: play rounds
        let monkey = Monkey::from(line);
        println!("{:?}", monkey);
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
