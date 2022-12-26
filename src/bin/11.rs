use std::collections::VecDeque;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Test {
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    nr: usize,
    inspections: usize,
    items: VecDeque<u64>,
    operation: String,
    test: Test,
}

enum DivisionStrategy {
    DivideByThree,
    Magic(u64),
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
            .parse::<usize>()
            .unwrap();
        let starting_items = iter
            .next()
            .unwrap()
            .replace("Starting items: ", "")
            .split(",")
            .map(|v| v.trim().parse::<u64>().unwrap())
            .collect_vec();
        let operation = iter.next().unwrap().replace("Operation: new = ", "");
        let test_divisible_by = iter
            .next()
            .unwrap()
            .replace("Test: divisible by ", "")
            .parse::<u64>()
            .unwrap();
        let test_if_true = iter
            .next()
            .unwrap()
            .replace("If true: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();
        let test_if_false = iter
            .next()
            .unwrap()
            .replace("If false: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();

        Monkey {
            nr,
            inspections: 0,
            items: VecDeque::from(starting_items),
            operation,
            test: Test {
                divisible_by: test_divisible_by,
                if_true: test_if_true,
                if_false: test_if_false,
            },
        }
    }
}

impl Monkey {
    /// Inspect a single item with a specific worry level.
    /// Returns the number of the next Monkey this item should be thrown to and the new worry level.
    fn inspect_item(&self, item: &u64, division: &DivisionStrategy) -> (usize, u64) {
        let mut worry_level = execute_operation(&self.operation.replace("old", &item.to_string()));

        match division {
            DivisionStrategy::DivideByThree => {
                worry_level = ((worry_level / 3) as f64).round() as u64;
            },
            DivisionStrategy::Magic(nr) => {
                worry_level %= nr;
            },
        }

        if worry_level % self.test.divisible_by == 0 {
            (self.test.if_true, worry_level)
        } else {
            (self.test.if_false, worry_level)
        }
    }
}

fn execute_operation(operation: &str) -> u64 {
    let mut chars = operation.split_whitespace();

    let left = chars.next().unwrap().parse::<u64>().unwrap();
    let operator = chars.next().unwrap();
    let right = chars.next().unwrap().parse::<u64>().unwrap();

    match operator {
        "+" => left + right,
        "*" => left * right,
        _ => panic!("unsupported operator {}", operator)
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|chunk| Monkey::from(chunk))
        .collect_vec()
}

fn play_round(monkeys: &mut Vec<Monkey>, division: &DivisionStrategy) {
    for i in 0..monkeys.len() {
        while let Some(item) = monkeys[i].items.pop_front() {
            let (next_monkey, worry_level) = monkeys[i].inspect_item(&item, division);

            // throw item with new worry level to next monkey
            monkeys[next_monkey].items.push_back(worry_level);

            // increase amount of inspections
            monkeys[i].inspections += 1;
        }
    }
}

fn play_rounds(monkeys: &mut Vec<Monkey>, count: usize, division: DivisionStrategy) {
    for _ in 0..count {
        play_round(monkeys, &division);
    }
}

fn find_most_active(mut monkeys: Vec<Monkey>) -> usize {
     // sort in descending order
     monkeys.sort_by(|m1, m2| m2.inspections.cmp(&m1.inspections));
     // return product of 2 most active monkeys
     monkeys[0].inspections * monkeys[1].inspections
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = parse_monkeys(input);

    play_rounds(&mut monkeys, 20, DivisionStrategy::DivideByThree);

    Some(find_most_active(monkeys))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = parse_monkeys(input);

    // In order to solve part 2, we have to 'find another way to keep your worry levels manageable'.
    // At first I tried to use BigInts (using the bigint-num crate), but that didn't work because the numbers would grow so big it would massively slow down the program at around 500 rounds.
    // I had to look this part up because the correct solution seems to be general knowledge to those who've encountered a similar problem before. And I'm also not a mathematician.
    // TL;DR: The idea here is that all the monkeys are doing modulo with the product of all divisors (which happen to be prime numbers) against you worry level.
    // More info here: https://fasterthanli.me/series/advent-of-code-2022/part-11#math-check
    let magic_nr = monkeys.iter().map(|x| x.test.divisible_by).product::<u64>();

    play_rounds(&mut monkeys, 10000, DivisionStrategy::Magic(magic_nr));

    Some(find_most_active(monkeys))
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
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
