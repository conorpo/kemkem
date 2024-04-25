//! Parameter Sets and Constants
//! 
//! - **ML-KEM-512**: K = 2, ETA_1 = 3, ETA_2 = 2, D_U = 10, D_V = 4
//! - **ML-KEM-768**: K = 3, ETA_1 = 2, ETA_2 = 2, D_U = 10, D_V = 4
//! - **ML-KEM-1024**: K = 4, ETA_1 = 2, ETA_2 = 2, D_U = 11, D_V = 5
//! 
//! *Constants are only used internally, but N represents the size of a ring (256 elements), Q is the prime modulus (3329), ZETA is the primitive root of unity (17)*
pub const N : usize = 256;
pub const Q : u16 = 3329;
pub const Q32: u32 = 3329;
pub const Q64: u64 = 3329;
pub const ZETA: u16 = 17;

/// Trait for adding parameter values to the 3 parameter set structs.
/// 
/// You can make new functions generic over all 3 parameter sets by using this trait as a bound.
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