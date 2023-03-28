use helping_turtles::safe_checker::{run, ValueType};
use helping_turtles::{BruteForceChecker, HashMapChecker, TwoPointersChecker};

fn main() {
    let input = include_str!("../example.txt").lines();
    let values: Vec<_> = input
        .map(|x| {
            x.parse::<ValueType>()
                .expect("Could not parse input value into u128.")
        })
        .collect();

    let number_of_values = values.len();

    let window_size = 5;
    assert!(number_of_values >= window_size);

    let mut initial_window = Vec::with_capacity(window_size);
    for value in values.iter().take(window_size) {
        initial_window.push(*value)
    }

    let result_from_hash_map =
        { run::<HashMapChecker>(window_size, &initial_window, number_of_values, &values) };

    let result_from_brute_force =
        { run::<BruteForceChecker>(window_size, &initial_window, number_of_values, &values) };

    let result_from_two_pointers =
        { run::<TwoPointersChecker>(window_size, &initial_window, number_of_values, &values) };

    assert_eq!(result_from_brute_force, result_from_hash_map);
    assert_eq!(result_from_brute_force, result_from_two_pointers);
}
