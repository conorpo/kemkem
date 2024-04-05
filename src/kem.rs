
use bitvec::prelude::*;

pub fn byte_encode(F: Polynomial, d: u8) -> BitVec {
    let mut bv = bitvec![u8, Msb0; 0; 32*d];

    for i in 0..256 {
        let mut a = F.data[i];
        for j in 0..d {
            let b = a % 2;
            bv.set(i*d + j, b);
            a = (a-b)/2;
        }
    }

    bv
}

pub fn byte_decode(bv: BitVec, d: u8) -> Polynomial {
    let mut F = Polynomial::new();
    let m = match d {
        12 => crate::Q,
        _ => 2u16.pow(d as u32)
    };

    bv.chunks(d).enumerate().for_each(|(i, chunk)| {
        let mut a = chunk.iter().fold(0, |acc, x| 2*acc + x as u16);
        F.data[i] = a % m;
    });

    F
}

pub fn key_gen() {
    let d = random_bytes(32);
}
