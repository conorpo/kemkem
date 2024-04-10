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
            TestRunner::<params::ML_KEM_512>::simulate();
        },
        "ML-KEM-768" => {
            TestRunner::<params::ML_KEM_768>::simulate();
        },
        "ML-KEM-1024" => {
            TestRunner::<params::ML_KEM_1024>::simulate();
        },
        _ => {
            panic!("Invalid parameter set");
        }
    }
}

struct TestRunner<Params: MlKemParams> {}

impl<Params: MlKemParams> TestRunner<Params> where
    [(); Params::eta_1]: ,
    [(); Params::eta_2]: ,
    [(); 64 * Params::eta_1]: ,
    [(); 384 * Params::k + 32]:
{
    fn simulate() => None {
        let (ek, dk) = mlkem::key_gen::<Params>();

        todo!();
    }

}

