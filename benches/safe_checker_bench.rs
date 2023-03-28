use criterion::{black_box, criterion_group, criterion_main, Criterion};

use helping_turtles::{run, BruteForceChecker, HashMapChecker, TwoPointersChecker};

fn generate_test_data() -> Vec<i128> {
    include_str!("../challenge_input.txt")
        .lines()
        .map(|x| x.parse::<i128>().unwrap())
        .collect()
}

fn bench_safe_checker(c: &mut Criterion) {
    let test_data = generate_test_data();
    let window_size = 100;
    let number_of_values = test_data.len();
    let values = test_data;
    let mut initial_window = Vec::with_capacity(window_size);
    for i in 0..window_size {
        initial_window.push(values[i])
    }

    c.bench_function("Brute Force", |b| {
        b.iter(|| {
            black_box(run::<BruteForceChecker>(
                window_size,
                &initial_window,
                number_of_values,
                &values,
            ));
        })
    });

    c.bench_function("Hash Map", |b| {
        b.iter(|| {
            black_box(run::<HashMapChecker>(
                window_size,
                &initial_window,
                number_of_values,
                &values,
            ));
        })
    });

    c.bench_function("Two Pointers", |b| {
        b.iter(|| {
            black_box(run::<TwoPointersChecker>(
                window_size,
                &initial_window,
                number_of_values,
                &values,
            ));
        })
    });
}

criterion_group!(benches, bench_safe_checker);
criterion_main!(benches);
