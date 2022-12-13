use std::collections::VecDeque;

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
    fn execution_cycle(&self) -> i32 {
        match self {
            Op::Noop => 1,
            Op::Addx(_) => 2,
        }
    }
}

struct Instruction {
    /// The operation to execute.
    op: Op,

    /// Number of the cycle when this instruction was added.
    cycle: i32,
}

impl Instruction {
    fn can_run(&self, cycle: i32) -> bool {
        let diff = cycle - self.cycle;
        diff == self.op.execution_cycle() // TODO: this doesn't work as intended when am add op is followed by a noop (debug to cycle 11 to see this issue in practice)
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut iter = input.lines();
    let mut x: i32 = 1;
    let mut cycle: i32 = 0;
    let mut total = 0;
    let mut callstack: VecDeque<Instruction> = VecDeque::new();

    loop {
        if let Some(op) = iter.next().map(Op::from) {
            // cycles += op.execution_cycle();
            callstack.push_back(Instruction { op, cycle });
        }

        if let Some(instruction) = callstack.front() {
            match instruction.op {
                Op::Noop => {
                    if instruction.can_run(cycle) {
                        callstack.pop_front();
                    }
                }
                Op::Addx(value) => {
                    if instruction.can_run(cycle) {
                        x += value;
                        callstack.pop_front();
                    }
                }
            }
        } else {
            break;
        }

        if (cycle - 20) % 40 == 0 {
            total += x * cycle;
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
