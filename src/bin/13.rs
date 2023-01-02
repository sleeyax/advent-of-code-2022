// I'm stuck because my code does exactly what the challenge description says and the example works, but my final answer is apparantly wrong ¯\_(ツ)_/¯
// I don't have much free time left to figure this out, so I'm leaving it as is for now.
// A nice solution which solves this problem in an idiomatic way is: https://www.ericburden.work/blog/2022/12/13/advent-of-code-2022-day-13/

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Integer(u32),
    Array(Vec<Value>),
}

impl Value {
    /// Unpacks the enum value to type `Vec<Value>`.
    /// If the `Value` is an integer it will be converted.
    fn unpack_array(&self) -> Vec<Value> {
        match self {
            Value::Integer(v) => vec![Value::Integer(*v)],
            Value::Array(v) => v.to_vec(),
        }
    }
}

/// Finds an array string within a longer nested array string notation.
///
/// Example:
/// ```
/// let input = "[[1, 2, 3]]";
/// let sub = find_sub_array(1, input);
/// println!("{:?}", sub); // [1, 2, 3]
/// ```
fn find_sub_array(index: usize, s: &str) -> String {
    let mut brackets_count = 0;
    let mut result = String::from("");

    for ch in s[index..s.len()].chars().into_iter() {
        match ch {
            '[' => brackets_count += 1,
            ']' => brackets_count -= 1,
            _ => {}
        };

        result.push(ch);

        if brackets_count == 0 {
            break;
        }
    }

    result
}

fn parse(input: &str) -> Vec<Value> {
    let mut result: Vec<Value> = Vec::new();
    let mut skip = 0;

    for (i, ch) in input.chars().enumerate() {
        if i <= skip {
            continue;
        }

        match ch {
            '[' => {
                let sub = find_sub_array(i, input);
                result.push(Value::Array(parse(sub.as_str())));
                skip = i + sub.len();
            }
            ']' | ',' | ' ' => {}
            _ => {
                if let Some(digit) = ch.to_digit(10) {
                    result.push(Value::Integer(digit));
                } else {
                    panic!("unexpected character '{}', expected digit!", ch);
                }
            }
        };
    }

    result
}

/// Compare two pairs and return whether they are in the right order.
fn compare(left: &Vec<Value>, right: &Vec<Value>, check_if_right_ran_out: bool) -> bool {
    let mut right_iter = right.iter();

    for l in left.iter() {
        if let Some(r) = right_iter.next() {
            match (l, r){
                (Value::Integer(l), Value::Integer(r)) => {
                    if r < l {
                        return false;
                    }
                },
                (Value::Array(l), Value::Array(r)) => {
                    if !compare(l, r, true) {
                        return false;
                    }
                },
                (l, r) => {
                    if !compare(&l.unpack_array(), &r.unpack_array(), false) {
                        return false;
                    }
                },
            };
        } else {
            // right side ran out of items
            return !check_if_right_ran_out;
        }
    }

    // left side ran out of items
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;

    for (i, pair) in input.split("\n\n").into_iter().enumerate() {
        let (left, right) = pair
            .split_once("\n")
            .map(|(left, right)| (parse(left.trim()), parse(right.trim())))
            .unwrap();

        if compare(&left, &right, true) {
            // println!("{}",i + 1);
            result += (i + 1) as u32;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_find_sub_array() {
        assert_eq!(find_sub_array(0, ""), "");
        assert_eq!(find_sub_array(0, "[]"), "[]");
        assert_eq!(find_sub_array(1, "[[]]"), "[]");
        assert_eq!(find_sub_array(1, "[[[]]]"), "[[]]");
        assert_eq!(find_sub_array(1, "[[1, 2, 3]]"), "[1, 2, 3]");
        assert_eq!(
            find_sub_array(1, "[[1, [2, [3, 4]], 5]]"),
            "[1, [2, [3, 4]], 5]"
        );
        assert_eq!(find_sub_array(5, "[[1, [2, [3, 4]], 5]]"), "[2, [3, 4]]");
    }

    #[test]
    fn test_parse() {
        let result = parse("[1, [2, [3, 4]], 5]");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Value::Integer(1));
        assert_eq!(
            result[1],
            Value::Array(vec![
                Value::Integer(2),
                Value::Array(vec![Value::Integer(3), Value::Integer(4)])
            ])
        );
        assert_eq!(result[2], Value::Integer(5));
    }
}
