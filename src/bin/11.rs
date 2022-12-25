use evalexpr::eval;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Test {
    divisible_by: isize,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    nr: isize,
    inspections: usize,
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

impl Monkey {
    /// Inspect a single item with a specific worry level.
    /// Returns the number of the next Monkey this item should be thrown to and the new worry level.
    fn inspect_item(&self, item: &isize) -> (usize, isize) {
        let worry_level = item;
        let worry_level = eval(&self.operation.replace("old", &worry_level.to_string()))
            .unwrap()
            .as_int()
            .unwrap();
        let worry_level = ((worry_level / 3) as f64).round() as isize;
        if worry_level % self.test.divisible_by == 0 {
            (self.test.if_true, worry_level)
        } else {
            (self.test.if_false, worry_level)
        }
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|chunk| Monkey::from(chunk))
        .collect_vec()
}

fn play_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        let monkey = monkeys[i].clone();

        for item in &monkey.items {
            let (next_monkey, worry_level) = monkey.inspect_item(item);

            // throw item with new worry level to next monkey
            let m = &mut monkeys[next_monkey];
            m.items.push(worry_level);

            // increase amount of inspections
            let m = &mut monkeys[i];
            m.inspections += 1;
        }

        // drop items with old worry level (remove them from our items list)
        let m = &mut monkeys[i];
        m.items.clear();
    }
}

fn play_rounds(monkeys: &mut Vec<Monkey>, count: usize) {
    for _ in 0..count {
        play_round(monkeys);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = parse_monkeys(input);

    play_rounds(&mut monkeys, 20);

    // println!("{:?}", &monkeys);

    // sort in descending order
    monkeys.sort_by(|m1, m2| m2.inspections.cmp(&m1.inspections));

    if let (Some(m1), Some(m2)) = (monkeys.get(0), monkeys.get(1)) {
        let monkey_business = m1.inspections * m2.inspections;
        Some(monkey_business)
    } else {
        panic!("expected at least 2 monkeys!");
    }
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
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
