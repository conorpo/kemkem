use crate::ring::*;
use crate::params;
use crate::crypt;

use bitvec::prelude::*;

pub fn sample_ntt(mut xof_stream: crypt::XOF) -> Ring {
    let mut j = 0;

    let mut a: Ring = Ring::new(RingRepresentation::NTT);

    while j < 256 {
        //Each iteration samples 3 unfiormly random bytes total
        let three_bytes = xof_stream.get_3_bytes();

        let d1 = three_bytes[0] as u16 + 256 * (three_bytes[1] % 16) as u16; // Uniform random sample of 12 bits
        let d2 = three_bytes[1] as u16 / 16 + 16 * three_bytes[2] as u16; // Uniform random sample of 12 bits

        if d1 < params::Q {
            a.data[j] = d1;
            j += 1;
        }

        if d2 < params::Q && j < 256 {
            a.data[j] = d2;
            j += 1;
        }
    }

    a
}

pub fn sample_poly_cbd<const ETA: usize>(byte_array: [u8; 64*ETA]) -> Ring 
    where [u8; 64*ETA]:
{
    let b = byte_array.view_bits::<Lsb0>();
    let mut f: Ring = Ring::new(RingRepresentation::Degree255);
    
    for (i, chunk) in b.chunks(2 * ETA).enumerate() {
        let mut x = 0u16;
        let mut y = 0u16;

        for j in 0..ETA {
            x += chunk[j] as u16;
            y += chunk[j + ETA] as u16;
        }

        f.data[i] = (x + params::Q - y) % params::Q;
    }

    f
}