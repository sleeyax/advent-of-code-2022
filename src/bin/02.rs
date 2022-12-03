#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum Outcome {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

#[derive(PartialEq)]
enum Strategy {
    /// Opponent based.
    One,
    /// Outcome based.
    Two,
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

impl From<&str> for Outcome {
    fn from(letter: &str) -> Self {
        match letter {
            "X" => Outcome::Lost,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Won,
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

fn predict_shape(opponent: Shape, outcome: Outcome) -> Shape {
    if outcome == Outcome::Draw {
        opponent
    } else if outcome == Outcome::Lost {
        match opponent {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    } else if outcome == Outcome::Won {
        match opponent {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    } else {
        panic!("unknown outcome specified!");
    }
}

fn play_round(left: &str, right: &str, strategy: &Strategy) -> u32 {
    let mut my_score: u32 = 0;
    let opponent = Shape::from(left);
    let me = if *strategy == Strategy::One { Shape::from(right)} else {predict_shape(Shape::from(left), Outcome::from(right))};
    my_score += me.to_score();
    my_score += play_game(opponent, me) as u32;
    my_score
}

fn play(input: &str, strategy: Strategy) -> Option<u32> {
    let mut my_total_score = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let opponent = split.get(0).unwrap();
        let me = split.get(1).unwrap();
        my_total_score += play_round(*opponent, *me, &strategy);
    }

    Some(my_total_score)
}

pub fn part_one(input: &str) -> Option<u32> {
    play(input, Strategy::One)
}

pub fn part_two(input: &str) -> Option<u32> {
    play(input, Strategy::Two)
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
        assert_eq!(part_two(&input), Some(12));
    }
}
