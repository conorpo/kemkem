/*
    Loads the correct parameters for the corresponding security level
*/
use std::marker::ConstParamTy;

pub const N : usize = 256;
pub const Q : u16 = 3329;

pub trait MlKemParams {
    const k: usize;
    const eta_1: usize;
    const eta_2: usize;
    const d_u: usize;
    const d_v: usize;
}

struct ML_KEM_512;
impl MlKemParams for ML_KEM_512 {
    const k: usize = 2;
    const eta_1: usize = 3;
    const eta_2: usize = 2;
    const d_u: usize = 10;
    const d_v: usize = 4;
}

struct ML_KEM_768;
impl MlKemParams for ML_KEM_768 {
    const k: usize = 3;
    const eta_1: usize = 2;
    const eta_2: usize = 2;
    const d_u: usize = 10;
    const d_v: usize = 4;
}

struct ML_KEM_1024;
impl MlKemParams for ML_KEM_1024 {
    const k: usize = 4;
    const eta_1: usize = 2;
    const eta_2: usize = 2;
    const d_u: usize = 11;
    const d_v: usize = 5;
}

// pub const ML_KEM_512: MlKemParams = MlKemParams {
//     k: 2,
//     eta_1: 3,
//     eta_2: 2,
//     d_u: 10,
//     d_v: 4
// };

// pub const ML_KEM_768: MlKemParams = MlKemParams {
//     k: 3,
//     eta_1: 2,
//     eta_2: 2,
//     d_u: 10,
//     d_v: 4
// };

// pub const ML_KEM_1024: MlKemParams = MlKemParams {
//     k: 4,
//     eta_1: 2,
//     eta_2: 2,
//     d_u: 11,
//     d_v: 5
// };