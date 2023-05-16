// Copyright (C) 2023-Present Divine Niiquaye Ibok.
// This file is part the of Yuniye Programming Language.

// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

mod benchmarks;

criterion::criterion_main! {
    benchmarks::example::benches
}
