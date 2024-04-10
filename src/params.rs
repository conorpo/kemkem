/*
    Loads the correct parameters for the corresponding security level
*/

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

pub struct MlKem512;
impl MlKemParams for MlKem512 {
    const k: usize = 2;
    const eta_1: usize = 3;
    const eta_2: usize = 2;
    const d_u: usize = 10;
    const d_v: usize = 4;
}

pub struct MlKem768;
impl MlKemParams for MlKem768 {
    const k: usize = 3;
    const eta_1: usize = 2;
    const eta_2: usize = 2;
    const d_u: usize = 10;
    const d_v: usize = 4;
}

pub struct MlKem1024;
impl MlKemParams for MlKem1024 {
    const k: usize = 4;
    const eta_1: usize = 2;
    const eta_2: usize = 2;
    const d_u: usize = 11;
    const d_v: usize = 5;
}