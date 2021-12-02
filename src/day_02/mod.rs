#[allow(dead_code)]
enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

#[allow(dead_code)]
fn move_submarine(commands: Vec<Command>) -> i64 {
    // (0, 0) -----------------> x
    // |
    // |
    // |
    // |
    // |
    // |
    // v
    // y
    let (x, y) = commands
        .into_iter()
        .fold((0, 0), |(x, y), command| match command {
            Command::Forward(val) => (x + val, y),
            Command::Down(val) => (x, y + val),
            Command::Up(val) => (x, y - val),
        });
    x * y
}

#[allow(dead_code)]
fn move_submarine_with_aim(commands: Vec<Command>) -> i64 {
    // (0, 0) -----------------> x
    // |
    // |
    // |
    // |
    // |
    // |
    // v
    // y
    let (x, y, _) = commands
        .into_iter()
        .fold((0, 0, 0), |(x, y, aim), command| match command {
            Command::Forward(val) => (x + val, y + aim * val, aim),
            Command::Down(val) => (x, y, aim + val),
            Command::Up(val) => (x, y, aim - val),
        });
    x * y
}

#[cfg(test)]
mod tests {
    use crate::common::read_input;
    use crate::day_02::Command;

    fn parse_input() -> Vec<Command> {
        read_input("day_02")
            .map(|line| match line.split(" ").collect::<Vec<&str>>()[..] {
                ["forward", val] => Command::Forward(val.parse().unwrap()),
                ["down", val] => Command::Down(val.parse().unwrap()),
                ["up", val] => Command::Up(val.parse().unwrap()),
                _ => panic!("Cannot parse line {:?}", line),
            })
            .collect()
    }

    mod part_1 {
        use crate::day_02::{move_submarine, tests::parse_input, Command};

        #[test]
        fn example_test() {
            assert_eq!(
                move_submarine(vec![
                    Command::Forward(5),
                    Command::Down(5),
                    Command::Forward(8),
                    Command::Up(3),
                    Command::Down(8),
                    Command::Forward(2),
                ]),
                150
            );
        }

        #[test]
        fn negative_test() {
            assert_eq!(
                move_submarine(vec![Command::Up(3), Command::Forward(1),]),
                -3
            );
        }

        #[test]
        fn solution() {
            assert_eq!(move_submarine(parse_input()), 1882980);
        }
    }
    mod part_2 {
        use crate::day_02::{move_submarine_with_aim, tests::parse_input, Command};

        #[test]
        fn example_test() {
            assert_eq!(
                move_submarine_with_aim(vec![
                    Command::Forward(5),
                    Command::Down(5),
                    Command::Forward(8),
                    Command::Up(3),
                    Command::Down(8),
                    Command::Forward(2),
                ]),
                900
            );
        }

        #[test]
        fn negative_test() {
            assert_eq!(
                move_submarine_with_aim(vec![Command::Up(3), Command::Forward(1),]),
                -3
            );
        }

        #[test]
        fn solution() {
            assert_eq!(move_submarine_with_aim(parse_input()), 1971232560);
        }
    }
}
