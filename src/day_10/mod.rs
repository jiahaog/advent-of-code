use std::collections::HashSet;

#[allow(dead_code)]
fn syntax_error_score(input: Vec<String>) -> usize {
    let (score, _) = find_corrupted(&input);
    score
}

fn find_corrupted(input: &Vec<String>) -> (usize, HashSet<usize>) {
    let mut score = 0;
    let mut corrupted = HashSet::new();

    for (i, line) in input.into_iter().enumerate() {
        let mut stack = Vec::new();

        for current in line.chars() {
            if stack.is_empty() {
                stack.push(current);
                continue;
            }

            if is_open(current) {
                stack.push(current);
                continue;
            }

            let prev = stack.last().unwrap();
            if to_closing(*prev) == current {
                stack.pop();
                continue;
            }

            score += to_corrupted_points(current);
            corrupted.insert(i);
            break;
        }
    }

    (score, corrupted)
}

fn to_corrupted_points(x: char) -> usize {
    match x {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unknown char {}", x),
    }
}

fn to_closing(x: char) -> char {
    match x {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Unknown char {}", x),
    }
}

fn is_open(x: char) -> bool {
    match x {
        '(' | '[' | '{' | '<' => true,
        _ => false,
    }
}

#[allow(dead_code)]
fn middle_incomplete(input: Vec<String>) -> usize {
    let (_, corrupted_lines_indices) = find_corrupted(&input);

    let incomplete_lines = input
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !corrupted_lines_indices.contains(i))
        .map(|(_, line)| line);

    let mut incomplete_scores = Vec::new();

    for line in incomplete_lines {
        let mut stack = Vec::new();

        for current in line.chars() {
            if stack.is_empty() {
                stack.push(current);
                continue;
            }

            if is_open(current) {
                stack.push(current);
                continue;
            }

            let prev = stack.last().unwrap();
            if to_closing(*prev) == current {
                stack.pop();
                continue;
            }

            panic!("line {} should be corrupted", line);
        }

        let score = stack.into_iter().rev().fold(0, |acc, x| {
            acc * 5
                + match to_closing(x) {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("Unexpected char {}", x),
                }
        });

        incomplete_scores.push(score);
    }

    incomplete_scores.sort();

    assert!(
        incomplete_scores.len() % 2 == 1,
        "Always odd number of scores to consider"
    );
    // Median.
    *incomplete_scores.get(incomplete_scores.len() / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::common::read_input;

    fn parse_input() -> impl Iterator<Item = String> {
        read_input("day_10")
    }

    const EXAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    mod part_1 {
        use super::EXAMPLE;

        use super::super::{syntax_error_score, tests::parse_input};

        #[test]
        fn example_test() {
            assert_eq!(
                syntax_error_score(EXAMPLE.lines().map(|x| x.to_string()).collect()),
                26397
            );
        }

        #[test]
        fn solution() {
            let input = parse_input().collect::<Vec<String>>();
            assert_eq!(syntax_error_score(input), 215229);
        }
    }

    mod part_2 {
        use super::EXAMPLE;

        use super::super::{middle_incomplete, tests::parse_input};

        #[test]
        fn example_test() {
            assert_eq!(
                middle_incomplete(EXAMPLE.lines().map(|x| x.to_string()).collect()),
                288957
            );
        }

        #[test]
        fn solution() {
            let input = parse_input().collect::<Vec<String>>();
            assert_eq!(middle_incomplete(input), 1105996483);
        }
    }
}
