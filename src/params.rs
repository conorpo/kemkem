/*
    Loads the correct parameters for the corresponding security level
*/
use std::marker::ConstParamTy;

pub const N : usize = 256;
pub const Q : u16 = 3329;
pub const Zeta: u16 = 17;

pub trait MlKemParams {
    const k: usize;
    const eta_1: usize;
    const eta_2: usize;
    const d_u: usize;
    const d_v: usize;
}

pub struct ML_KEM_512;
impl MlKemParams for ML_KEM_512 {
    const k: Self::T = 2;
    const eta_1: Self::T = 3;
    const eta_2: Self::T = 2;
    const d_u: Self::T = 10;
    const d_v: Self::T = 4;
}

pub struct ML_KEM_768;
impl MlKemParams for ML_KEM_768 {
    const k: usize = 3;
    const eta_1: usize = 2;
    const eta_2: usize = 2;
    const d_u: usize = 10;
    const d_v: usize = 4;
}

pub struct ML_KEM_1024;
impl MlKemParams for ML_KEM_1024 {
    const k: usize = 4;
    const eta_1: usize = 2;
    const eta_2: usize = 2;
    const d_u: usize = 11;
    const d_v: usize = 5;
}