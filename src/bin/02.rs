#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

impl Shape {
    fn to_score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl From<&str> for Shape {
    fn from(letter: &str) -> Self {
        match letter {
            "X" | "A" => Shape::Rock,
            "Y" | "B" => Shape::Paper,
            "Z" | "C" => Shape::Scissors,
            _ => panic!("unknown letter {} specified!", letter)
        }
    }
}

fn play_game(opponent: Shape, player: Shape) -> Outcome {
    if opponent == player {
        Outcome::Draw
    } else if (player == Shape::Rock && opponent == Shape::Scissors) || (player == Shape::Scissors && opponent == Shape::Paper) || (player == Shape::Paper && opponent == Shape::Rock) {
        Outcome::Won
    } else {
        Outcome::Lost
    }
}

fn play_round(opponent: Shape, me: Shape) -> u32 {
    let mut my_score: u32 = 0;
    my_score += me.to_score();
    my_score += play_game(opponent, me) as u32;
    my_score
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut my_total_score = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let opponent = split.get(0).unwrap();
        let me = split.get(1).unwrap();
        my_total_score += play_round(Shape::from(*opponent), Shape::from(*me));
    }

    Some(my_total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
