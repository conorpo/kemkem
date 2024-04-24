// #![feature(generic_const_exprs)]

use criterion::{criterion_group, criterion_main, Criterion};

use kemkem::mlkem::*;
use kemkem::params::*;

use kemkem::serialize::*;
type PARAMS = MlKem768;

fn bench_keygen(criterion: &mut Criterion) {
    criterion.bench_function("KeyGen", |b| b.iter(|| {
        let (_ek, _dk) = key_gen::<PARAMS>();
    }));
}

fn bench_encaps(criterion: &mut Criterion) {
    criterion.bench_function("Encaps", |b| {
        b.iter_batched(|| key_gen::<PARAMS>().0, |ek| {
            let (_key, _c) = encaps::<PARAMS>(ek);
        }, criterion::BatchSize::SmallInput);
    });
}

fn bench_decaps(criterion: &mut Criterion) {
    criterion.bench_function("Decaps", |b| {
        b.iter_batched(|| {
            let (ek, dk) = key_gen::<PARAMS>();
            let (key, c) = encaps::<PARAMS>(ek);
            (c, dk, key)
        }, |(c, dk, key) | {
            let key_prime = decaps::<PARAMS>(c, dk);
            assert_eq!(key, key_prime);
        }, criterion::BatchSize::SmallInput);
    });
}

criterion_group!(bench_768, bench_keygen, bench_encaps, bench_decaps);
criterion_main!(bench_768);
