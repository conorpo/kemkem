/*
    Loads the correct parameters for the corresponding security level
*/

pub const N : usize = 256;
pub const Q : u16 = 3329;
pub const ZETA: u16 = 17;

pub trait MlKemParams {
    const K: usize;
    const ETA_1: usize;
    const ETA_2: usize;
    const D_U: usize;
    const D_V: usize;
}

pub struct MlKem512;
impl MlKemParams for MlKem512 {
    const K: usize = 2;
    const ETA_1: usize = 3;
    const ETA_2: usize = 2;
    const D_U: usize = 10;
    const D_V: usize = 4;
}

pub struct MlKem768;
impl MlKemParams for MlKem768 {
    const K: usize = 3;
    const ETA_1: usize = 2;
    const ETA_2: usize = 2;
    const D_U: usize = 10;
    const D_V: usize = 4;
}

pub struct MlKem1024;
impl MlKemParams for MlKem1024 {
    const K: usize = 4;
    const ETA_1: usize = 2;
    const ETA_2: usize = 2;
    const D_U: usize = 11;
    const D_V: usize = 5;
}