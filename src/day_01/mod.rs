#[allow(dead_code)]
fn num_increases(input: Vec<i64>) -> usize {
    let (increases, _) = input
        .into_iter()
        .fold((0, i64::MIN), |(increases, prev), current| {
            if current > prev {
                (increases + 1, current)
            } else {
                (increases, current)
            }
        });

    increases - 1
}

#[allow(dead_code)]
fn num_increases_with_window(input: Vec<i64>) -> usize {
    num_increases(to_three_measurement_window_sums(input))
}

fn to_three_measurement_window_sums(input: Vec<i64>) -> Vec<i64> {
    let a = &input[0..];
    let b = &input[1..];
    let c = &input[2..];

    a.into_iter()
        .zip(b)
        .into_iter()
        .zip(c)
        .into_iter()
        .map(|((a, b), c)| a + b + c)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::common::read_input;

    fn parse_input() -> Vec<i64> {
        read_input("day_01")
            .map(|line| line.parse::<i64>().unwrap())
            .collect()
    }

    mod part_1 {
        use crate::day_01::{num_increases, tests::parse_input};

        #[test]
        fn example_test() {
            assert_eq!(num_increases(vec![1]), 0);
            assert_eq!(
                num_increases(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263,]),
                7
            );
        }

        #[test]
        fn solution() {
            let input = parse_input();
            assert_eq!(num_increases(input), 1832);
        }
    }
    mod part_2 {
        use crate::day_01::{num_increases_with_window, tests::parse_input};

        #[test]
        fn example_test() {
            assert_eq!(num_increases_with_window(vec![1, 1, 1]), 0);
            assert_eq!(
                num_increases_with_window(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263,]),
                5
            );
        }

        #[test]
        fn solution() {
            let input = parse_input();
            assert_eq!(num_increases_with_window(input), 1858);
        }
    }
}
