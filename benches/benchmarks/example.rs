// Copyright (C) 2023-Present Divine Niiquaye Ibok.
// This file is part the of Yuniye Programming Language.

// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use std::time::Duration;
use criterion::{criterion_group, Criterion};

fn compare_rust_vector(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rust Vector");
    let mut data = Vec::new();

    for i in 0..10_000_000 {
        data.push(i);
    }
    
    fn bench_push() {
        let mut vec: Vec<usize> = Vec::new();

        for i in 0..10_000_000 {
            vec.push(i);
        }
    }

    fn bench_index_get(vec: &Vec<usize>) {
        for i in 1..10_000_000 {
            assert_eq!(Some(&i), vec.get(i));
        }
    }

    group.bench_function("Push", |b| b.iter(bench_push));
    group.bench_with_input("Index Get", &data,  |b, v| b.iter(|| bench_index_get(v)));
}

criterion_group!{
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(15));
    targets = compare_rust_vector
}
