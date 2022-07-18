use criterion::{criterion_group, criterion_main, Criterion};
use eint::{Eint, E256};
use fast_eint::c_impl;
use fast_eint::rust_impl;
use fast_eint::widening_mul_256;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use std::slice;

const BATCH_RUNS: usize = 128;

pub fn fast_batch_mul256_benchmark(c: &mut Criterion) {
    c.bench_function("fast batch mul256", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(10000);
        let mut buf = vec![0u8; (32 + 32 + 64) * BATCH_RUNS];
        rng.fill_bytes(&mut buf);

        b.iter(|| {
            widening_mul_256(
                &mut buf,
                (32 + 32) * BATCH_RUNS,
                0,
                32 * BATCH_RUNS,
                BATCH_RUNS,
            );
        })
    });
}

pub fn fast_batch_mul256_c_benchmark(c: &mut Criterion) {
    c.bench_function("fast batch mul256 (c-impl)", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(10000);
        let mut buf = vec![0u8; (32 + 32 + 64) * BATCH_RUNS];
        rng.fill_bytes(&mut buf);

        let (xy, w) = buf.split_at_mut((32 + 32) * BATCH_RUNS);
        b.iter(|| {
            c_impl::widening_mul_256(
                w,
                &xy[0..32 * BATCH_RUNS],
                &xy[32 * BATCH_RUNS..(32 + 32) * BATCH_RUNS],
                BATCH_RUNS,
            );
        })
    });
}

pub fn fast_batch_mul256_rust_benchmark(c: &mut Criterion) {
    c.bench_function("fast batch mul256 (rust-impl)", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(10000);
        let mut buf = vec![0u8; (32 + 32 + 64) * BATCH_RUNS];
        rng.fill_bytes(&mut buf);

        let (xy, w) = buf.split_at_mut((32 + 32) * BATCH_RUNS);
        b.iter(|| {
            rust_impl::widening_mul_256(
                w,
                &xy[0..32 * BATCH_RUNS],
                &xy[32 * BATCH_RUNS..(32 + 32) * BATCH_RUNS],
                BATCH_RUNS,
            );
        })
    });
}

pub fn normal_batch_mul256_benchmark(c: &mut Criterion) {
    c.bench_function("normal batch mul256", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(10000);
        let mut buf = vec![0u8; (32 + 32 + 64) * BATCH_RUNS];
        rng.fill_bytes(&mut buf);

        b.iter(|| {
            for i in 0..BATCH_RUNS {
                let a = E256::get_unsafe(&buf[i * 32..i * 32 + 32]);
                let b =
                    E256::get_unsafe(&buf[32 * BATCH_RUNS + i * 32..32 * BATCH_RUNS + i * 32 + 32]);

                let (lo, hi) = a.widening_mul_u(b);

                lo.put(
                    &mut buf[(32 + 32) * BATCH_RUNS + i * 32..(32 + 32) * BATCH_RUNS + i * 32 + 32],
                );
                hi.put(
                    &mut buf[(32 + 32) * BATCH_RUNS + i * 32 + 32
                        ..(32 + 32) * BATCH_RUNS + i * 32 + 64],
                );
            }
        })
    });
}

pub fn fast_single_mul256_benchmark(c: &mut Criterion) {
    c.bench_function("fast single mul256", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(20000);
        let mut buf = vec![0u8; 32 + 32 + 64];
        rng.fill_bytes(&mut buf);

        b.iter(|| {
            widening_mul_256(&mut buf, 32 + 32, 0, 32, 1);
        })
    });
}

pub fn fast_single_mul256_c_benchmark(c: &mut Criterion) {
    c.bench_function("fast single mul256(c-impl)", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(20000);
        let mut buf = vec![0u8; 32 + 32 + 64];
        rng.fill_bytes(&mut buf);

        let (xy, w) = buf.split_at_mut(32 + 32);
        let (x, y) = xy.split_at_mut(32);
        b.iter(|| {
            // widening_mul_256(&mut buf, 32 + 32, 0, 32, 1);
            c_impl::widening_mul_256(w, x, y, 1)
        })
    });
}

pub fn fast_single_mul256_rust_benchmark(c: &mut Criterion) {
    c.bench_function("fast single mul256(rust-impl)", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(20000);
        let mut buf = vec![0u8; 32 + 32 + 64];
        rng.fill_bytes(&mut buf);

        let (xy, w) = buf.split_at_mut(32 + 32);
        let (x, y) = xy.split_at_mut(32);

        b.iter(|| rust_impl::widening_mul_256(w, x, y, 1))
    });
}

pub fn fast_single_mul256_rust2_benchmark(c: &mut Criterion) {
    c.bench_function("fast single mul256(rust-impl alternative)", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(20000);
        let mut buf = vec![0u8; 32 + 32 + 64];
        rng.fill_bytes(&mut buf);

        let (xy, w) = buf.split_at_mut(32 + 32);
        let (x, y) = xy.split_at_mut(32);

        let w = unsafe {
            let ptr = w.as_ptr() as *mut u64;
            slice::from_raw_parts_mut(ptr, 8)
        };
        let x = unsafe {
            let ptr = x.as_ptr() as *mut u64;
            slice::from_raw_parts(ptr, 4)
        };
        let y = unsafe {
            let ptr = y.as_ptr() as *mut u64;
            slice::from_raw_parts(ptr, 4)
        };
        assert_eq!(w.len(), 8);
        assert_eq!(x.len(), 4);
        assert_eq!(y.len(), 4);
        b.iter(|| rust_impl::widening_mul(w, x, y, 8))
    });
}

pub fn normal_single_mul256_benchmark(c: &mut Criterion) {
    c.bench_function("normal single mul256", |b| {
        let mut rng = ChaCha20Rng::seed_from_u64(20000);
        let mut buf = vec![0u8; 32 + 32 + 64];
        rng.fill_bytes(&mut buf);

        b.iter(|| {
            let a = E256::get_unsafe(&buf[0..32]);
            let b = E256::get_unsafe(&buf[32..64]);

            let (lo, hi) = a.widening_mul_u(b);

            lo.put(&mut buf[64..96]);
            hi.put(&mut buf[96..128]);
        })
    });
}

criterion_group!(
    benches,
    normal_single_mul256_benchmark,
    fast_single_mul256_benchmark,
    fast_single_mul256_c_benchmark,
    fast_single_mul256_rust_benchmark,
    fast_single_mul256_rust2_benchmark,
    normal_batch_mul256_benchmark,
    fast_batch_mul256_benchmark,
    fast_batch_mul256_c_benchmark,
    fast_batch_mul256_rust_benchmark,
);
criterion_main!(benches);
