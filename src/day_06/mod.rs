use std::collections::{HashMap, VecDeque};

// Every 7 days, a fish produces another fish.
//
// Day 0 => 1 fish
// Day 1 => 1 fish
// ...
// Day 6 => 1 fish
// Day 7 => 2 fish
//
// A new fish takes 7 + 2 days to produce a fish for the first cycle.

const DAYS_TO_REPRODUCE: i64 = 7;
const DAYS_TO_REPRODUCE_FIRST_CYCLE: i64 = DAYS_TO_REPRODUCE + 2;

#[allow(dead_code)]
fn laternfish_after_n_days_brute_force(fishes: &Vec<i64>, n: i64) -> i64 {
    let mut fishes = fishes.clone();

    for _ in 0..n {
        update_one_day(&mut fishes);
    }

    fishes.len() as i64
}

fn update_one_day(fishes: &mut Vec<i64>) {
    let mut new_timers = Vec::new();

    for timer in fishes.iter_mut() {
        if *timer == 0 {
            *timer = DAYS_TO_REPRODUCE - 1;
            new_timers.push(DAYS_TO_REPRODUCE_FIRST_CYCLE - 1);
            continue;
        }

        let new_timer = *timer - 1;
        *timer = new_timer;
    }

    fishes.extend(new_timers.into_iter());
}

#[allow(dead_code)]
fn laternfish_after_n_days(fishes: &Vec<i64>, n: i64) -> i64 {
    let mut cache = HashMap::new();

    fishes
        .into_iter()
        .fold(0, |acc, fish| acc + count_fishes(&mut cache, n, *fish))
}

// I did not think of this.
#[allow(dead_code)]
fn laternfish_after_n_days_simple(fishes: &Vec<i64>, n: i64) -> i64 {
    let mut counts = fishes.into_iter().fold(
        // Plus one because counts range from 0 to 8 inclusive.
        VecDeque::from_iter(vec![0; DAYS_TO_REPRODUCE_FIRST_CYCLE as usize]),
        |mut counts, current_fish| {
            *counts
                .get_mut(*current_fish as usize)
                .expect("Timers should go from only 0 to 8") += 1;
            counts
        },
    );

    for _ in 0..n {
        let new_parents = counts.pop_front().unwrap();
        *counts.get_mut(DAYS_TO_REPRODUCE as usize - 1).unwrap() += new_parents;
        counts.push_back(new_parents);
    }

    counts.into_iter().sum()
}

/// Counts the fishes that will exist in the pool after t, given a fish with `fish_time`.
fn count_fishes(cache: &mut HashMap<(i64, i64), i64>, t: i64, time_to_reproduction: i64) -> i64 {
    let memoize_key = (t, time_to_reproduction);
    if cache.contains_key(&memoize_key) {
        return *cache.get(&memoize_key).unwrap();
    }
    assert!(time_to_reproduction <= 8);
    assert!(time_to_reproduction >= 0);

    let mut result = 1;

    // Child is only produced when current_t is < 0, not when it is at 0.
    let time_to_produce_first_child = time_to_reproduction + 1;

    let remaining_time_for_children = t - time_to_produce_first_child;
    if remaining_time_for_children < 0 {
        return result;
    }

    // If remaining_time_for_children < DAYS_TO_REPRODUCE, there will be zero
    // generations. Add 1 for the first generation.
    let children_generations = remaining_time_for_children / DAYS_TO_REPRODUCE + 1;

    for i in 0..children_generations {
        let time_for_generation = remaining_time_for_children - i * DAYS_TO_REPRODUCE;

        result += count_fishes(
            cache,
            time_for_generation,
            DAYS_TO_REPRODUCE_FIRST_CYCLE - 1,
        );
    }

    cache.insert(memoize_key, result);

    result
}

#[cfg(test)]
mod tests {
    use crate::common::read_input;

    fn parse_input() -> Vec<i64> {
        read_input("day_06")
            .flat_map(|line| {
                line.split(",")
                    .map(|char| char.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect()
    }

    const EXAMPLE: [i64; 5] = [3, 4, 3, 1, 2];

    mod part_1 {
        use crate::day_06::{laternfish_after_n_days_brute_force, laternfish_after_n_days_simple};

        use super::super::laternfish_after_n_days;
        use super::{parse_input, EXAMPLE};

        #[test]
        fn empty_test() {
            assert_eq!(laternfish_after_n_days(&Vec::new(), 10), 0,);
            assert_eq!(laternfish_after_n_days(&vec![8], 8), 1,);
        }

        #[test]
        fn small_test() {
            for (fishes, days) in vec![
                (vec![8], 0),
                (vec![8], 1),
                (vec![8], 2),
                (vec![8], 3),
                (vec![8], 4),
                (vec![8], 5),
                (vec![8], 6),
                (vec![8], 7),
                (vec![2], 0),
                (vec![1], 1),
                (vec![1], 2),
                (vec![6], 0),
                (vec![6], 1),
                (vec![6], 2),
                (vec![6], 3),
                (vec![6], 4),
                (vec![6], 5),
                (vec![6], 6),
                (vec![6], 7),
                (vec![6], 14),
                (vec![6], 15),
                (vec![6], 16),
            ] {
                assert_eq!(
                    laternfish_after_n_days(&fishes, days),
                    laternfish_after_n_days_brute_force(&fishes, days),
                    "failed for {:?}",
                    (fishes, days)
                );

                assert_eq!(
                    laternfish_after_n_days_simple(&fishes, days),
                    laternfish_after_n_days_brute_force(&fishes, days),
                    "failed for {:?}",
                    (fishes, days)
                );
            }
        }

        #[test]
        fn example_test() {
            assert_eq!(
                laternfish_after_n_days(&EXAMPLE.into_iter().collect(), 80),
                5934,
            );
        }

        #[test]
        fn solution() {
            let input = parse_input();
            assert_eq!(laternfish_after_n_days(&input, 80), 366057,);

            assert_eq!(laternfish_after_n_days_brute_force(&input, 80), 366057,);
        }
    }

    mod part_2 {
        use crate::day_06::laternfish_after_n_days_simple;

        use super::super::laternfish_after_n_days;
        use super::{parse_input, EXAMPLE};

        // Too slow for brute force.

        #[test]
        fn example_test() {
            assert_eq!(
                laternfish_after_n_days(&EXAMPLE.into_iter().collect(), 256),
                26984457539,
            );
        }

        #[test]
        fn solution() {
            let input = parse_input();
            assert_eq!(laternfish_after_n_days(&input, 256), 1653559299811,);

            assert_eq!(laternfish_after_n_days_simple(&input, 256), 1653559299811,);
        }
    }
}
