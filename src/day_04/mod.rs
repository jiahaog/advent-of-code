use std::collections::HashSet;

/// Assumptions:
///
/// - Board is 5 x 5.
/// - Board cannot have duplicate numbers.
/// - There must be a winning board after all numbers have been drawn.

#[derive(Debug)]
struct BingoBoard([[u8; 5]; 5]);

impl BingoBoard {
    fn new(matrix: [[u8; 5]; 5]) -> Self {
        Self(matrix)
    }

    fn iter(&self) -> BingoBoardIter {
        BingoBoardIter {
            bingo_board: &self,
            next: 0,
        }
    }

    /// Returns `Some(indices)` of the marks if the board is a winning board.
    fn winning_mark_indices(&self, draws: &[u8]) -> Option<Vec<usize>> {
        let draws_set: HashSet<u8> = HashSet::from_iter(draws.into_iter().map(|draw| *draw));

        let marks = self
            .iter()
            .enumerate()
            .filter_map(|(i, val)| {
                if draws_set.contains(&val) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        let is_winning = marks
            .iter()
            // First 5 indices to count marked columns, next 5 to count rows.
            .fold([0; 10], |mut indices, i| {
                indices[i % 5] += 1;
                indices[i / 5 + 5] += 1;
                indices
            })
            .into_iter()
            .any(|count| count == 5);

        if is_winning {
            Some(marks)
        } else {
            None
        }
    }

    fn score(&self, mark_indices: Vec<usize>, last_draw: u8) -> u64 {
        let mark_indices: HashSet<usize> = HashSet::from_iter(mark_indices.into_iter());

        let board_sum: u64 = self.iter().map(|x| x as u64).sum();
        let board_sum_marked: u64 = self
            .iter()
            .enumerate()
            .filter_map(|(i, val)| {
                if mark_indices.contains(&i) {
                    Some(val)
                } else {
                    None
                }
            })
            .map(|x| x as u64)
            .sum();
        (board_sum - board_sum_marked) * last_draw as u64
    }
}

struct BingoBoardIter<'a> {
    bingo_board: &'a BingoBoard,
    next: u8,
}

impl<'a> Iterator for BingoBoardIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self
            .bingo_board
            .0
            .get((self.next / 5) as usize)
            .and_then(|row| row.get((self.next % 5) as usize))
            .map(|val| *val);

        self.next += 1;
        result
    }
}

#[allow(dead_code)]
fn bingo_first_to_win_score(draws: Vec<u8>, matrices: Vec<[[u8; 5]; 5]>) -> u64 {
    let bingo_boards = matrices
        .into_iter()
        .map(|matrix| BingoBoard::new(matrix))
        .collect::<Vec<BingoBoard>>();

    for i in 1..draws.len() {
        let current_draws = &draws[0..i];

        let maybe_winning_score = bingo_boards.iter().find_map(|board| {
            if let Some(mark_indices) = board.winning_mark_indices(current_draws) {
                let last_draw = *current_draws
                    .last()
                    .expect("There should be at least one draw");

                Some(board.score(mark_indices, last_draw))
            } else {
                None
            }
        });

        if let Some(winning_score) = maybe_winning_score {
            return winning_score;
        }
    }

    panic!("No winning boards found after all draws")
}

#[allow(dead_code)]
fn bingo_last_to_win_score(draws: Vec<u8>, matrices: Vec<[[u8; 5]; 5]>) -> u64 {
    let bingo_boards = matrices
        .into_iter()
        .map(|matrix| BingoBoard::new(matrix))
        .collect::<Vec<BingoBoard>>();

    let mut alive_board_indices: HashSet<usize> =
        HashSet::from_iter((0..bingo_boards.len()).into_iter());

    for i in 1..draws.len() {
        let current_draws = &draws[0..i];

        if alive_board_indices.len() == 1 {
            let last_board = bingo_boards
                .get(*alive_board_indices.iter().next().unwrap())
                .unwrap();

            let last_draw = *current_draws
                .last()
                .expect("There should be at least one draw");

            if let Some(mark_indices) = last_board.winning_mark_indices(current_draws) {
                return last_board.score(mark_indices, last_draw);
            } else {
                // Last board but not won yet.
                continue;
            }
        }

        for (i, board) in bingo_boards.iter().enumerate() {
            // TODO: There's probably a data structure that faster removes
            if !alive_board_indices.contains(&i) {
                continue;
            }
            if let Some(_) = board.winning_mark_indices(current_draws) {
                alive_board_indices.remove(&i);
            }
        }
    }

    panic!("No winning boards found after all draws")
}

#[cfg(test)]
mod tests {
    use crate::common::read_input;

    fn parse_input_string(input: &str) -> (Vec<u8>, Vec<[[u8; 5]; 5]>) {
        let chunks = input.split("\n\n").collect::<Vec<&str>>();

        let draws = chunks[0].split(",").map(|x| x.parse().unwrap()).collect();

        let mut boards = Vec::new();

        for raw_board in chunks[1..].into_iter() {
            let mut board = [[0; 5]; 5];
            for (j, row) in raw_board.split("\n").enumerate() {
                for (i, val) in row.split_whitespace().enumerate() {
                    board[j][i] = val.parse().unwrap();
                }
            }

            boards.push(board);
        }

        (draws, boards)
    }

    fn parse_input() -> String {
        read_input("day_04").collect::<Vec<String>>().join("\n")
    }

    const EXAMPLE_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    mod part_1 {
        use crate::day_04::{
            bingo_first_to_win_score,
            tests::{parse_input, EXAMPLE_INPUT},
        };

        use super::parse_input_string;

        #[test]
        fn example_test() {
            let (draws, matrices) = parse_input_string(EXAMPLE_INPUT);

            assert_eq!(bingo_first_to_win_score(draws, matrices), 4512);
        }

        #[test]
        fn solution() {
            let (draws, matrices) = parse_input_string(&parse_input());

            assert_eq!(bingo_first_to_win_score(draws, matrices), 33462);
        }
    }
    mod part_2 {
        use crate::day_04::{
            bingo_last_to_win_score,
            tests::{parse_input, parse_input_string, EXAMPLE_INPUT},
        };

        #[test]
        fn example_test() {
            let (draws, matrices) = parse_input_string(EXAMPLE_INPUT);

            assert_eq!(bingo_last_to_win_score(draws, matrices), 1924);
        }

        #[test]
        fn solution() {
            let (draws, matrices) = parse_input_string(&parse_input());

            assert_eq!(bingo_last_to_win_score(draws, matrices), 30070);
        }
    }
}
