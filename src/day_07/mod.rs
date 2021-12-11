/// Minimize `abs(a1 - x) + abs(a2 - x) + ...`.
#[allow(dead_code)]
fn fuel_to_align(positions: &Vec<i64>) -> i64 {
    fuel_to_align_using_fuel_calculation(|steps| steps, positions)
}

/// Minimize `calc(abs(a1 - x) + calc(a2 - x) + ...`
/// where `calc(n)` is `1 + 2 + ... + n`.
#[allow(dead_code)]
fn fuel_to_align_non_constant_fuel(positions: &Vec<i64>) -> i64 {
    fuel_to_align_using_fuel_calculation(|steps| (steps * (steps + 1)) / 2, positions)
}

fn fuel_to_align_using_fuel_calculation(
    fuel_calculation: fn(i64) -> i64,
    positions: &Vec<i64>,
) -> i64 {
    if positions.is_empty() {
        return 0;
    }

    let best_position_lower = *positions.iter().min().unwrap();
    let best_position_upper = *positions.iter().max().unwrap();

    (best_position_lower..best_position_upper + 1)
        .into_iter()
        .map(|guess| {
            positions.iter().fold(0, |acc, x| {
                let steps = (x - guess).abs();
                acc + fuel_calculation(steps)
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::common::read_input;

    fn parse_input() -> Vec<i64> {
        read_input("day_07")
            .flat_map(|line| {
                line.split(",")
                    .map(|char| char.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect()
    }

    const EXAMPLE: [i64; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    mod part_1 {
        use super::super::fuel_to_align;
        use super::{parse_input, EXAMPLE};

        #[test]
        fn empty_test() {
            assert_eq!(fuel_to_align(&Vec::new()), 0,);
        }

        #[test]
        fn example_test() {
            assert_eq!(fuel_to_align(&EXAMPLE.into_iter().collect()), 37,);
        }

        #[test]
        fn solution() {
            let input = parse_input();
            assert_eq!(fuel_to_align(&input), 351901,);
        }
    }

    mod part_2 {
        use super::super::fuel_to_align_non_constant_fuel;
        use super::{parse_input, EXAMPLE};

        #[test]
        fn empty_test() {
            assert_eq!(fuel_to_align_non_constant_fuel(&Vec::new()), 0,);
        }

        #[test]
        fn example_test() {
            assert_eq!(
                fuel_to_align_non_constant_fuel(&EXAMPLE.into_iter().collect()),
                168,
            );
        }

        #[test]
        fn solution() {
            let input = parse_input();
            assert_eq!(fuel_to_align_non_constant_fuel(&input), 101079875,);
        }
    }
}
