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

fn solve(input: &str) -> i32 {
    let mut iter = input.lines();
    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut total = 0;
    let mut callstack: VecDeque<Instruction> = VecDeque::new();
    let mut pixels: Vec<&str> = Vec::new();

    // What I had in mind initially was to parse each instruction at runtime an expand the amount of cycles in the loop as we continue to read the input line by line.
    // But Rust doesn't allow modifications to a range while looping over it!
    // Thus this while loop instead.
    // Alternatively we could create a fn to count the number of cycles beforehand, though this isn't really required (as of part 1).
    // Perhaps my initial idea is still possible some other way though, but I haven't discovered that solution yet...
    loop {
        // Parse next instruction.
        if let Some(op) = iter.next().map(Op::from) {
            let mut instruction = Instruction { op, cycle };

            if let Some(last_instruction) = callstack.back() {
                instruction.cycle = last_instruction.cycle;
            }

            instruction.cycle += op.execution_cycle();

            callstack.push_back(instruction);
        }

        // We're done if there are no more instructions to execute.
        if callstack.len() == 0 {
            break;
        }

        // Totals must be calculated at the START of this cycle.
        if (cycle as i32 - 20) % 40 == 0 {
            let signal_strength = x * cycle as i32;
            total += signal_strength;
        }

        // Execute instructions at the END of this cycle.
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

        // The CRT draws a pixel during each cycle.
        let pos = cycle as i32 % 40;
        if pos - 1 == x || pos == x || pos + 1 == x {
            pixels.push("#");
        } else {
            pixels.push(".");
        }
        if (cycle + 1) % 40 == 0 {
            println!("{}", pixels.join(""));
            pixels.clear();
        }

        // Clean the stack by removing executed instructions.
        callstack.retain(|instruction| !instruction.can_run(cycle));

        cycle += 1;
    }

    total
}

pub fn part_one(input: &str) -> Option<i32> {
   Some(solve(input))
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(solve(input))
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
        assert_eq!(part_two(&input), Some(13140));
    }
}
