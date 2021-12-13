use std::collections::{HashMap, HashSet};

// This is bad and I feel bad. I didn't read the question properly to understand
// that the first 10 signal patterns are combinations from 0 - 9. This
// implementation does a backtracking search to find the solution instead, which
// is slow. (Maybe dynamic programming could make this faster)

#[allow(dead_code)]
fn times_1_4_7_8_appear(input: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    input.into_iter().fold(0, |acc, (_, outputs)| {
        acc + outputs.into_iter().fold(0, |acc, output| {
            acc + (output.len() == 2 || output.len() == 4 || output.len() == 3 || output.len() == 7)
                as usize
        })
    })
}

#[allow(dead_code)]
fn all_output_values_sum(input: &Vec<(Vec<String>, Vec<String>)>) -> u64 {
    let mut result = 0;

    for (signal_pattern, outputs) in input {
        let mut current_result = 0;

        let mut solution: HashMap<char, HashSet<u8>> =
            HashMap::from_iter(('a'..='g').into_iter().map(|char| {
                let initial_possibilities = HashSet::from_iter(1..=8);
                (char, initial_possibilities)
            }));
        let patterns = signal_pattern.into_iter().chain(outputs.into_iter());

        // This just prunes the tree and is not strictly needed for correctness.
        // It affects performance greatly though.
        for pattern in patterns {
            match pattern.len() {
                // Signal for 1.
                2 => {
                    for char in pattern.chars() {
                        let char_possibilities = solution.get_mut(&char).unwrap();
                        keep(char_possibilities, vec![2, 3]);
                    }
                }
                // Signal for 4.
                4 => {
                    for char in pattern.chars() {
                        let char_possibilities = solution.get_mut(&char).unwrap();
                        keep(char_possibilities, vec![6, 2, 7, 3]);
                    }
                }
                // Signal for 7.
                3 => {
                    for char in pattern.chars() {
                        let char_possibilities = solution.get_mut(&char).unwrap();
                        keep(char_possibilities, vec![1, 2, 3]);
                    }
                }
                // Signal for 8.
                // TODO: Might as well do nothing here?
                7 => {
                    for char in pattern.chars() {
                        let char_possibilities = solution.get_mut(&char).unwrap();
                        keep(char_possibilities, vec![1, 2, 3, 4, 5, 6, 7, 8]);
                    }
                }
                _ => (),
            }
        }

        let valid_check = |solution| {
            for signal in signal_pattern.into_iter().chain(outputs.into_iter()) {
                if signal_to_number(&solution, signal).is_none() {
                    return false;
                }
            }
            return true;
        };

        if let Some(solution) = backtrack(&valid_check, solution.clone()) {
            for signal in outputs {
                current_result *= 10;
                current_result += signal_to_number(&solution, signal).unwrap() as u64;
            }
        } else {
            panic!("No solution found");
        };

        result += current_result;
    }
    result
}

fn keep(possibilities: &mut HashSet<u8>, to_keep: Vec<u8>) {
    possibilities.retain(|x| to_keep.contains(x));

    assert!(possibilities.len() > 0);
}

//  1111
// 6    2
// 6    2
//  7777
// 5    3
// 5    3
//  4444
fn positions_to_number(positions: &HashSet<u8>) -> Option<u8> {
    if positions == &HashSet::from_iter(vec![1, 2, 3, 4, 5, 6]) {
        Some(0)
    } else if positions == &HashSet::from_iter(vec![2, 3]) {
        Some(1)
    } else if positions == &HashSet::from_iter(vec![1, 2, 7, 5, 4]) {
        Some(2)
    } else if positions == &HashSet::from_iter(vec![1, 2, 7, 3, 4]) {
        Some(3)
    } else if positions == &HashSet::from_iter(vec![6, 7, 2, 3]) {
        Some(4)
    } else if positions == &HashSet::from_iter(vec![1, 6, 7, 3, 4]) {
        Some(5)
    } else if positions == &HashSet::from_iter(vec![1, 6, 5, 4, 3, 7]) {
        Some(6)
    } else if positions == &HashSet::from_iter(vec![1, 2, 3]) {
        Some(7)
    } else if positions == &HashSet::from_iter(vec![1, 6, 7, 3, 4, 5, 2]) {
        Some(8)
    } else if positions == &HashSet::from_iter(vec![1, 6, 7, 2, 3, 4]) {
        Some(9)
    } else {
        None
    }
}

fn backtrack(
    valid_check: &dyn Fn(HashMap<char, HashSet<u8>>) -> bool,
    possibilities: HashMap<char, HashSet<u8>>,
) -> Option<HashMap<char, HashSet<u8>>> {
    let maybe_candidate = possibilities
        .iter()
        // K is a definite solution and is not a candidate.
        .filter(|(_, v)| v.len() != 1)
        .min_by_key(|(_, p)| p.len());

    if maybe_candidate.is_none() {
        if valid_check(possibilities.clone()) {
            return Some(possibilities);
        } else {
            return None;
        }
    }
    let (candidate, guesses) = maybe_candidate.unwrap();

    for guess in guesses {
        let mut next_possibilities = possibilities.clone();

        *next_possibilities.get_mut(candidate).unwrap() = HashSet::from_iter(vec![*guess]);

        for (char, positions) in next_possibilities.iter_mut() {
            if char == candidate {
                continue;
            }

            positions.retain(|x| x != guess);
        }

        if let Some(solution) = backtrack(valid_check, next_possibilities) {
            return Some(solution);
        }
    }

    None
}

fn signal_to_number(solution: &HashMap<char, HashSet<u8>>, signal: &str) -> Option<u8> {
    let solution: HashMap<char, u8> = HashMap::from_iter(
        solution
            .into_iter()
            .map(|(k, v)| (*k, *v.into_iter().next().unwrap())),
    );

    let positions: HashSet<u8> =
        HashSet::from_iter(signal.chars().map(|char| *solution.get(&char).unwrap()));

    positions_to_number(&positions)
}

#[cfg(test)]
mod tests {
    use crate::common::read_input;

    fn parse_input_str(input: &str) -> (Vec<String>, Vec<String>) {
        let (raw_signal_patterns, raw_outputs) = input.split_once("|").unwrap();

        let signal_patterns = raw_signal_patterns
            .trim()
            .split_whitespace()
            // TODO: Not sure how to split a string without copying.
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let outputs = raw_outputs
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        (signal_patterns, outputs)
    }

    fn parse_input() -> Vec<(Vec<String>, Vec<String>)> {
        read_input("day_08")
            .map(|line| parse_input_str(&line))
            .collect()
    }

    const EXAMPLE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    mod part_1 {
        use crate::day_08::tests::parse_input_str;

        use super::super::times_1_4_7_8_appear;
        use super::{parse_input, EXAMPLE};

        #[test]
        fn example_test() {
            let example_input = EXAMPLE.lines().map(|line| parse_input_str(line)).collect();
            assert_eq!(times_1_4_7_8_appear(&example_input), 26,);
        }

        #[test]
        fn solution() {
            assert_eq!(times_1_4_7_8_appear(&parse_input()), 421,);
        }
    }

    mod part_2 {
        use crate::day_08::all_output_values_sum;
        use crate::day_08::tests::parse_input_str;

        use super::{parse_input, EXAMPLE};

        #[test]
        fn example_small_test() {
            let input = vec![parse_input_str("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")];
            assert_eq!(all_output_values_sum(&input), 5353);
        }

        #[test]
        fn example_test() {
            let example_input = EXAMPLE.lines().map(|line| parse_input_str(line)).collect();
            assert_eq!(all_output_values_sum(&example_input), 61229,);
        }

        #[test]
        fn solution() {
            assert_eq!(all_output_values_sum(&parse_input()), 986163,);
        }
    }
}
