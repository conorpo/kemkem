#![feature(generic_const_exprs)]

extern crate kemkem;
use kemkem::params::*;
use kemkem::mlkem::*;


fn main() {
    let temp_param_set = "ML-KEM-1024";

    match temp_param_set {
        "ML-KEM-512" => {
            TestRunner::<MlKem512>::simulate();
        },
        "ML-KEM-768" => {
            TestRunner::<MlKem768>::simulate();
        },
        "ML-KEM-1024" => {
            TestRunner::<MlKem1024>::simulate();
        },
        _ => {
            panic!("Invalid parameter set");
        }
    }
}

struct TestRunner<PARAMS: MlKemParams> {
    _phantom: std::marker::PhantomData<PARAMS>
}

impl <PARAMS: MlKemParams> TestRunner<PARAMS> where
    [(); PARAMS::ETA_1]: ,
    [(); PARAMS::ETA_2]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 384 * PARAMS::K + 32]: ,
    [(); 32*(PARAMS::D_U * PARAMS::K + PARAMS::D_V)]: ,
    [(); 64 * PARAMS::ETA_2]: ,
{
    fn simulate() {
        let (ek, dk) = key_gen::<PARAMS>();

        // dbg!(&ek.0, &ek.1);
        // dbg!(&dk.0, &dk.1, &dk.2, &dk.3);

        let (shared_key, c) = encaps::<PARAMS>(ek);

        // dbg!(&shared_key);
        // dbg!(&c.0, &c.1);

        let shared_key_prime = decaps::<PARAMS>(c, dk);

        // dbg!(&shared_key_prime);

        assert_eq!(shared_key, shared_key_prime);
    }
}