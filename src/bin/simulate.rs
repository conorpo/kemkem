//Runs the algorithim one time
#![feature(generic_const_exprs)]

use kemkem::{mlkem::*, params::*};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let paramset = args.get(1).expect("Missing parameter set");

    match paramset.as_str() {
        "ML-KEM-512" => simulate::<MlKem512>(),
        "ML-KEM-768" => simulate::<MlKem768>(),
        "ML-KEM-1024" => simulate::<MlKem1024>(),
        _ => panic!("Invalid parameter set")
    };
}


fn simulate<PARAMS: MlKemParams>() where
    [(); 768 * PARAMS::K + 96]: , 
    [(); PARAMS::ETA_1]: ,
    [(); PARAMS::ETA_2]: ,
    [(); 384 * PARAMS::K + 32]: ,
    [(); 32*(PARAMS::D_U * PARAMS::K + PARAMS::D_V)]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 64 * PARAMS::ETA_2]:
{
    let (ek, dk) = key_gen::<PARAMS>();

    let (key, c) = encaps::<PARAMS>(ek);

    let key_prime = decaps::<PARAMS>(c, dk);

    assert_eq!(key, key_prime);

    println!("Simulation successful");
}