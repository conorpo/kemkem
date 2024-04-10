#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

const Q: i16 = 3329;

use kem::key_gen;

use crypt::*;
use params::MlKemParams;

mod mlkem;
mod crypt;
mod bits;
mod kpke;
mod sample;
mod polynomial;
mod params;

fn main() {
    let temp_param_set = "ML-KEM-1024";

    match temp_param_set {
        "ML-KEM-512" => {
            TestRunner::<params::MlKem512>::simulate();
        },
        "ML-KEM-768" => {
            TestRunner::<params::MlKem768>::simulate();
        },
        "ML-KEM-1024" => {
            TestRunner::<params::MlKem1024>::simulate();
        },
        _ => {
            panic!("Invalid parameter set");
        }
    }
}

struct TestRunner<params: MlKemParams> {}

impl<params: MlKemParams> TestRunner<params> where
    [(); params::eta_1]: ,
    [(); params::eta_2]: ,
    [(); 64 * params::eta_1]: ,
    [(); 384 * params::k + 32]:
{
    fn simulate() => None {
        let (ek, dk) = mlkem::key_gen::<params>();

        todo!();
    }

}

