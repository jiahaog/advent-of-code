use std::fmt::Debug;

const DIRECTIONS: [[isize; 2]; 8] = [
    [0, -1],
    [1, 0],
    [0, 1],
    [-1, 0],
    // Diagonals.
    [1, 1],
    [1, -1],
    [-1, -1],
    [-1, 1],
];

const FLASH_THRESHOLD: u8 = 10;

struct Grid(Vec<Vec<u8>>);

#[cfg(debug_assertions)]
impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().map(|row| write!(f, "{:?}\n", row)).collect()
    }
}

impl Grid {
    fn new(vec: Vec<Vec<u8>>) -> Self {
        Self(vec)
    }

    fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        let num_rows = self.0.len();
        let num_cols = self.0.first().unwrap().len();

        (0..num_rows).flat_map(move |j| (0..num_cols).map(move |i| (i, j)))
    }

    fn size(&self) -> usize {
        self.positions().collect::<Vec<_>>().len()
    }

    fn increase(&mut self, x: isize, y: isize) -> usize {
        let mut result = 0;

        if y < 0 || x < 0 {
            return result;
        }
        let x = x as usize;
        let y = y as usize;

        if y >= self.0.len() || x >= self.0[y].len() {
            return result;
        }

        self.0[y][x] += 1;

        // Don't use `>` check so this takes care of the condition that one cell
        // can only flash once until it is reset.
        if self.0[y][x] != FLASH_THRESHOLD {
            return result;
        }

        result += 1;

        result
            + DIRECTIONS
                .into_iter()
                .map(|[delta_x, delta_y]| self.increase(x as isize + delta_x, y as isize + delta_y))
                .sum::<usize>()
    }

    fn reset_flashes(&mut self) {
        self.positions().for_each(|(x, y)| {
            if self.0[y][x] >= FLASH_THRESHOLD {
                self.0[y][x] = 0;
            }
        });
    }
}

#[allow(dead_code)]
fn flashes_after_100_steps(input: Vec<Vec<u8>>) -> usize {
    let mut result = 0;

    let mut grid = Grid::new(input);

    for _ in 0..100 {
        result += grid
            .positions()
            .map(|(x, y)| grid.increase(x as isize, y as isize))
            .sum::<usize>();

        grid.reset_flashes();
    }
    result
}

#[allow(dead_code)]
fn synchronize_step(input: Vec<Vec<u8>>) -> usize {
    let mut grid = Grid::new(input);

    for i in 0..usize::MAX {
        let flashes = grid
            .positions()
            .map(|(x, y)| grid.increase(x as isize, y as isize))
            .sum::<usize>();

        if flashes == grid.size() {
            // Steps are 1-indexed.
            return i + 1;
        }

        grid.reset_flashes();
    }

    panic!("Could not find synchronize step")
}

#[cfg(test)]
mod tests {
    use crate::common::read_input_to_string;

    fn parse_input() -> Vec<Vec<u8>> {
        let input_str = read_input_to_string("day_11");
        parse_input_str(&input_str)
    }

    fn parse_input_str(input: &str) -> Vec<Vec<u8>> {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect()
    }

    const EXAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    mod part_1 {
        use super::{parse_input_str, EXAMPLE};

        use super::super::{flashes_after_100_steps, tests::parse_input};

        #[test]
        fn example_test() {
            assert_eq!(flashes_after_100_steps(parse_input_str(EXAMPLE)), 1656);
        }

        #[test]
        fn solution() {
            let input = parse_input();
            assert_eq!(flashes_after_100_steps(input), 1620);
        }
    }
    mod part_2 {
        use super::{parse_input_str, EXAMPLE};

        use super::super::{synchronize_step, tests::parse_input};

        #[test]
        fn example_test() {
            assert_eq!(synchronize_step(parse_input_str(EXAMPLE)), 195);
        }

        #[test]
        fn solution() {
            let input = parse_input();
            assert_eq!(synchronize_step(input), 371);
        }
    }
}
