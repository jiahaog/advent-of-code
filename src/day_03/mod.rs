#[allow(dead_code)]
fn binary_diagnostic(input: Vec<String>) -> u64 {
    let gamma = common_in_columns(true, &input);
    let epsilon = common_in_columns(false, &input);

    gamma * epsilon
}

fn common_in_columns(is_finding_most_common: bool, lines: &Vec<String>) -> u64 {
    // TODO remove this clone.
    Diagnostic::new(lines.clone())
        .into_iter()
        .map(|column| {
            let half = column.len() / 2;
            let sum: usize = column
                .into_iter()
                .map(|char| match char {
                    '0' => 0,
                    '1' => 1,
                    x => panic!("Unexpected character in line: {}", x),
                })
                .fold(0, |acc, current| acc + current as usize);

            // Unclear from the question what is the "most common bit" if there
            // are equal `1`s and `0`s.
            let is_most_common_one = sum > half;

            // A - is_finding_most_common
            // B - is_most_common_one
            //
            // A B Result
            // ----------
            // 1 1 1
            // 1 0 0
            // 0 1 0
            // 0 0 1
            !(is_finding_most_common ^ is_most_common_one)
        })
        .fold(0, |acc, current| (acc << 1) + current as u64)
}

struct Diagnostic {
    lines: Vec<String>,
    i: usize,
}

impl Diagnostic {
    fn new(lines: Vec<String>) -> Self {
        Self { lines, i: 0 }
    }
}

/// Iterator where each item is a vec of chars in the nth column.
impl Iterator for Diagnostic {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self
            .lines
            .iter()
            .filter_map(|line| line.chars().nth(self.i))
            .collect::<Vec<char>>();

        self.i += 1;

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::read_input;

    fn parse_input() -> Vec<String> {
        read_input("day_03").collect()
    }

    mod part_1 {
        use crate::day_03::{binary_diagnostic, tests::parse_input};

        fn vec_string_ref_to_vec_string(vector: Vec<&str>) -> Vec<String> {
            vector.into_iter().map(|x| x.to_string()).collect()
        }

        #[test]
        fn example_test() {
            assert_eq!(
                binary_diagnostic(vec_string_ref_to_vec_string(vec![
                    "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100",
                    "10000", "11001", "00010", "01010",
                ])),
                198
            );
        }

        #[test]
        fn epsilon_leading_zero() {
            assert_eq!(
                binary_diagnostic(vec_string_ref_to_vec_string(vec!["01", "01", "01"])),
                // gamma * epsilon.
                0b1 * 0b10
            );
        }

        #[test]
        fn solution() {
            assert_eq!(binary_diagnostic(parse_input()), 4006064);
        }
    }
}
