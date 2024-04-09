use crate::polynomial::*;
use crate::params;
use crate::crypt;

pub fn SampleNTT(mut xof_stream: crypt::XOF) -> Polynomial {
    let mut i = 0;
    let mut j = 0;

    let mut a: Polynomial = Polynomial::new(PolynomialType::NTT);

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

        i += 3;
    }

    a
}

pub fn SamplePolyCBD_2 (byte_array: [u8; 128]) -> Polynomial {
    let b = bytes_to_bits(byte_array);

    let mut f: Polynomial = Polynomial::Ring([0; 256]);

    for i in 0..256 {
        let x = b[4*i] + b[4*i + 1];
        let y = b[4*i + 2] + b[4*i + 3];
        f[i] = (x - y) % params::Q;
    }

    f
}

// possible make samplepolycbd use a const generic
pub fn SamplePolyCBD_3 (byte_array: [u8; 192]) -> Polynomial {
    let b = bytes_to_bits(byte_array);

    let mut f: Polynomial = Polynomial::Ring([0; 256]);

    for i in 0..256 {
        let x = b[6*i] + b[6*i + 1] + b[6*i + 2];
        let y = b[6*i + 3] + b[6*i + 4] + b[6*i + 5];
        f[i] = (x - y) % params::Q;
    }

    f
}