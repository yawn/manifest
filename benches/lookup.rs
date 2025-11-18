#![allow(unexpected_cfgs)]

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use test_crate::{
    BENCH_DEPRECATED_FEATURE, BENCH_LOGIN_FAILED, BENCH_PASSWORD_RESET, BENCH_USER_LOGIN,
    lookup_message,
};

#[cfg(not(no_inline_lookup))]
const CONFIG: &str = "inline(always)";

#[cfg(no_inline_lookup)]
const CONFIG: &str = "no_inline";

fn bench_lookup_best_case(c: &mut Criterion) {
    eprintln!("==> Running benchmarks with: {}", CONFIG);
    let mut group = c.benchmark_group("lookup_best_case");

    group.bench_function("first_message", |b| {
        b.iter(|| {
            let msg = lookup_message(black_box(BENCH_USER_LOGIN));
            black_box(msg);
        });
    });

    group.finish();
}

fn bench_lookup_mid_case(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup_mid_case");

    group.bench_function("second_message", |b| {
        b.iter(|| {
            let msg = lookup_message(black_box(BENCH_LOGIN_FAILED));
            black_box(msg);
        });
    });

    group.finish();
}

fn bench_lookup_worst_case(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup_worst_case");

    group.bench_function("last_message", |b| {
        b.iter(|| {
            let msg = lookup_message(black_box(BENCH_DEPRECATED_FEATURE));
            black_box(msg);
        });
    });

    group.finish();
}

fn bench_lookup_multiple(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup_multiple");

    group.bench_function("three_lookups", |b| {
        b.iter(|| {
            let msg1 = lookup_message(black_box(BENCH_USER_LOGIN));
            let msg2 = lookup_message(black_box(BENCH_LOGIN_FAILED));
            let msg3 = lookup_message(black_box(BENCH_PASSWORD_RESET));
            black_box((msg1, msg2, msg3));
        });
    });

    group.finish();
}

fn bench_lookup_constant_known(c: &mut Criterion) {
    let mut group = c.benchmark_group("lookup_constant_known");

    group.bench_function("known_constant_no_black_box", |b| {
        b.iter(|| {
            let msg = lookup_message(BENCH_USER_LOGIN);
            black_box(msg);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_lookup_best_case,
    bench_lookup_mid_case,
    bench_lookup_worst_case,
    bench_lookup_multiple,
    bench_lookup_constant_known,
);

criterion_main!(benches);
