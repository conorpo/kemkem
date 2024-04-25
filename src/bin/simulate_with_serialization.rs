//Runs the algorithim one time
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use kemkem::{mlkem::*, params::*, serialize::*};

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
    // Party A
    let (ek, dk) = key_gen::<PARAMS>();

    let ek = ek.serialize();
    let dk = dk.serialize();

    // Party B
    let ek = MlKemEncapsulationKey::<{PARAMS::K}>::deserialize(&ek);
    
    let (key, c) = encaps::<PARAMS>(ek);

    let c = c.serialize();

    // Party A
    let c = MlKemCyphertext::<{PARAMS::K}, {PARAMS::D_U}, {PARAMS::D_V}>::deserialize(&c);
    let dk = MlKemDecapsulationKey::<{PARAMS::K}>::deserialize(&dk);

    let key_prime = decaps::<PARAMS>(c, dk);


    assert_eq!(key, key_prime);
    println!("Success!, Keys match!");
}