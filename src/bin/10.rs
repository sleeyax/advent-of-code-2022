use std::{collections::VecDeque};

#[derive(Clone, Copy)]
enum Op {
    Noop,
    Addx(i32),
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s {
            "noop" => Self::Noop,
            value => {
                let amount = value.split(" ").last().unwrap();
                Self::Addx(amount.parse::<i32>().unwrap())
            }
        }
    }
}

impl Op {
    fn execution_cycle(&self) -> usize {
        match self {
            Op::Noop => 1,
            Op::Addx(_) => 2,
        }
    }
}

#[derive(Clone)]
struct Instruction {
    /// The operation to execute.
    op: Op,

    /// Number of the cycle when this instruction should execute.
    cycle: usize,
}

impl Instruction {
    fn can_run(&self, cycle: usize) -> bool {
        cycle == self.cycle
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut iter = input.lines();
    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut total = 0;
    let mut callstack: VecDeque<Instruction> = VecDeque::new();

    // What I had in mind initially was to parse each instruction at runtime an expand the amount of cycles in the loop as we continue to read the input line by line.
    // But Rust doesn't allow modifications to a range while looping over it!
    // Therefore, I've implemented it so we count the number of cycles beforehand instead.
    // This is not very performant and perhaps my initial idea is still possible some other way though, but I haven't discovered it yet...
    loop {
        if let Some(op) = iter.next().map(Op::from) {
            let mut instruction = Instruction { op, cycle };

            if let Some(last_instruction) = callstack.back() {
                instruction.cycle = last_instruction.cycle;
            }

            instruction.cycle += op.execution_cycle();

            callstack.push_back(instruction);
        }

        if callstack.len() == 0 {
            break;
        }

        for instruction in callstack.iter() {
            if !instruction.can_run(cycle) {
                continue;
            }

            match instruction.op {
                Op::Addx(value) => {
                    x += value;
                },
                _ => {}
            };
        }

        callstack.retain(|instruction| !instruction.can_run(cycle));

        if (cycle as i32 - 20) % 40 == 0 {
            let signal_strength = x * cycle as i32;
            total += signal_strength;
            // TODO: find out why the 220th cycle has an invalid value for x
            if cycle == 220 {
                break;
            }
        }

        cycle += 1;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
