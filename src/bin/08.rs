fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>()
}

fn is_visible(current: &u32, range: Vec<u32>) -> bool {
    range.iter().all(|v| v < current)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    let grid = parse_grid(input);

    for (r, row) in grid.iter().enumerate() {
        for (c, column) in row.iter().enumerate() {
            // edges are always visible
            if r == 0 || c == 0 || r == row.len() - 1 || c == grid.len() - 1 {
                count += 1;
                continue;
            }

            let left = row[0..c].to_vec();
            let right = row[c + 1..row.len()].to_vec();
            let top = grid.iter().take(r).map(|vec| vec[c]).collect::<Vec<_>>();
            let bottom = grid
                .iter()
                .skip(r + 1)
                .map(|vec| vec[c])
                .collect::<Vec<_>>();

            // println!("{:?} {} {:?}", left, column, right);

            if is_visible(column, left)
                || is_visible(column, right)
                || is_visible(column, top)
                || is_visible(column, bottom)
            {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_one_custom() {
        let cwd = env::current_dir().unwrap();
        let filepath = cwd.join("src").join("examples").join("08_top_bottom.txt");
        let f = fs::read_to_string(filepath);
        let input = f.expect("could not open input file");
        assert_eq!(part_one(&input), Some(23));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
