use std::collections::{BinaryHeap, HashSet};

const DIRECTIONS: [[isize; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

#[allow(dead_code)]
fn risk_level_sum(input: &Vec<Vec<i8>>) -> u64 {
    low_points(input)
        .into_iter()
        .map(|(x, y)| 1 + *input.get(y as usize).unwrap().get(x as usize).unwrap() as u64)
        .sum::<u64>()
}

fn low_points(input: &Vec<Vec<i8>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let is_low_point = DIRECTIONS.iter().all(|vector| {
                let (neighbor_x, neighbor_y) = (x as isize + vector[0], y as isize + vector[1]);
                if neighbor_x < 0
                    || neighbor_y < 0
                    || neighbor_x >= row.len() as isize
                    || neighbor_y >= input.len() as isize
                {
                    return true;
                }

                let neighbor = input
                    .get(neighbor_y as usize)
                    .unwrap()
                    .get(neighbor_x as usize)
                    .unwrap();

                cell < neighbor
            });

            if is_low_point {
                result.push((x, y));
            }
        }
    }
    result
}

#[allow(dead_code)]
fn basin_sizes(input: &Vec<Vec<i8>>) -> u64 {
    let mut basins = BinaryHeap::new();

    for low_point in low_points(input) {
        let mut size = 0;

        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let mut queue = MinHeap::new();
        queue.push((
            // Cannot use i8::MIN because we are going to negate this in the
            // MinHeap and it will overflow.
            -i8::MAX,
            (low_point.0 as isize, low_point.1 as isize),
        ));

        while let Some((prev, (x, y))) = queue.pop() {
            if x < 0
                || y < 0
                || y >= input.len() as isize
                || x >= input.get(y as usize).unwrap().len() as isize
            {
                continue;
            }

            if visited.contains(&(x, y)) {
                continue;
            }

            let current = *input.get(y as usize).unwrap().get(x as usize).unwrap();
            if current == 9 {
                continue;
            }

            visited.insert((x, y));

            if current <= prev {
                continue;
            }

            size += 1;

            for vector in DIRECTIONS {
                let (next_x, next_y) = (x as isize + vector[0], y as isize + vector[1]);
                queue.push((current, (next_x, next_y)));
            }
        }

        basins.push(size);
    }

    basins.into_iter().take(3).fold(1, |acc, x| acc * x)
}

struct MinHeap(BinaryHeap<(i8, (isize, isize))>);

impl MinHeap {
    fn new() -> Self {
        Self(BinaryHeap::new())
    }
    fn push(&mut self, (depth, coordinates): (i8, (isize, isize))) {
        self.0.push((-depth, coordinates));
    }

    fn pop(&mut self) -> Option<(i8, (isize, isize))> {
        self.0
            .pop()
            .map(|(negative_depth, coordinates)| (-negative_depth, coordinates))
    }
}

#[cfg(test)]
mod tests {
    use crate::common::read_input_to_string;

    fn parse_input_str(input: &str) -> Vec<Vec<i8>> {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap() as i8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn parse_input() -> Vec<Vec<i8>> {
        let input = read_input_to_string("day_09");
        parse_input_str(&input)
    }

    const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    mod part_1 {
        use super::{super::risk_level_sum, parse_input_str};
        use super::{parse_input, EXAMPLE};

        #[test]
        fn example_test() {
            let input = parse_input_str(EXAMPLE);
            assert_eq!(risk_level_sum(&input), 15);
        }

        #[test]
        fn solution() {
            let input = parse_input();
            assert_eq!(risk_level_sum(&input), 468);
        }
    }

    mod part_2 {
        use super::{super::basin_sizes, parse_input_str};
        use super::{parse_input, EXAMPLE};

        #[test]
        fn example_test() {
            let input = parse_input_str(EXAMPLE);
            assert_eq!(basin_sizes(&input), 1134);
        }

        #[test]
        fn solution() {
            let input = parse_input();
            assert_eq!(basin_sizes(&input), 1280496);
        }
    }
}
