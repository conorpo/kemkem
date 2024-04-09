#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

const Q: i16 = 3329;

use kem::key_gen;

use crypt::*;

mod mlkem;
mod crypt;
mod bits;
mod kpke;
mod sample;
mod polynomial;
mod params;

fn main() {
    let temp_param_set = "ML-KEM-1024";

    let (ek_pke, s) = match temp_param_set {
        "ML-KEM-512" => mlkem::key_gen<params::ML_KEM_512>(),
        "ML-KEM-768" => mlkem::key_gen<params::ML_KEM_768>(),
        "ML-KEM-1024" => mlkem::key_gen<params::ML_KEM_1024>(),
        _ => panic!("Invalid parameter set")
    };
}
