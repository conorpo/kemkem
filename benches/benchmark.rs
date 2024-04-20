#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use kemkem::mlkem::*;
use kemkem::params::*;
use kemkem::serialize::*;

fn bench_mlkem<PARAMS: MlKemParams>(criterion: &mut Criterion) where
    [(); 768 * PARAMS::K + 96]: , 
    [(); PARAMS::ETA_1]: ,
    [(); PARAMS::ETA_2]: ,
    [(); 384 * PARAMS::K + 32]: ,
    [(); 32*(PARAMS::D_U * PARAMS::K + PARAMS::D_V)]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 64 * PARAMS::ETA_2]:
{
    criterion.bench_function(&format!("ML-KEM-{}", 256 * PARAMS::K), |b| b.iter(|| {
        let (ek, dk) = key_gen::<PARAMS>();
        let (key, c) = encaps::<PARAMS>(ek);
        let key_prime = decaps::<PARAMS>(c, dk);

        assert_eq!(key, key_prime);

        black_box(key_prime);
    }));
}

fn bench_mlkem_with_serialization<PARAMS: MlKemParams>(criterion: &mut Criterion) where
    [(); 768 * PARAMS::K + 96]: , 
    [(); PARAMS::ETA_1]: ,
    [(); PARAMS::ETA_2]: ,
    [(); 384 * PARAMS::K + 32]: ,
    [(); 32*(PARAMS::D_U * PARAMS::K + PARAMS::D_V)]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 64 * PARAMS::ETA_2]:
{
    criterion.bench_function(&format!("ML-KEM-{} w/ Serializaiton", 256 * PARAMS::K), |b| b.iter(|| {
        let (ek, dk) = key_gen::<PARAMS>();
        let ek = ek.serialize();
        let dk = dk.serialize();
        let ek = MlkemEncapsulationKey::<{PARAMS::K}>::deserialize(&ek);
        let dk = MlkemDecapsulationKey::<{PARAMS::K}>::deserialize(&dk);
        let (key, c) = encaps::<PARAMS>(ek);
        let c = c.serialize();
        let c = MlKemCyphertext::<{PARAMS::K}, {PARAMS::D_U}, {PARAMS::D_V}>::deserialize(&c);
        let key_prime = decaps::<PARAMS>(c, dk);

        assert_eq!(key, key_prime);

        black_box(key_prime);
    }));
}

fn bench_512(criterion: &mut Criterion) {
    bench_mlkem::<MlKem512>(criterion);
}

fn bench_768(criterion: &mut Criterion) {
    bench_mlkem::<MlKem768>(criterion);
}

fn bench_1024(criterion: &mut Criterion) {
    bench_mlkem::<MlKem1024>(criterion);
}

criterion_group!(benches_without_serialization, bench_512, bench_768, bench_1024);

fn bench_512_with_serialization(criterion: &mut Criterion) {
    bench_mlkem_with_serialization::<MlKem512>(criterion);
}

fn bench_768_with_serialization(criterion: &mut Criterion) {
    bench_mlkem_with_serialization::<MlKem768>(criterion);
}

fn bench_1024_with_serialization(criterion: &mut Criterion) {
    bench_mlkem_with_serialization::<MlKem1024>(criterion);
}

criterion_group!(benches_with_serialization, bench_512_with_serialization, bench_768_with_serialization, bench_1024_with_serialization);

criterion_main!(benches_without_serialization, benches_with_serialization);
