use crate::crypt;
use crate::params::*;
use crate::ring::*;
use crate::sample;
use crate::bits::*;
use crate::serialize::{ByteDecode, BitOrder, Decompress};

use bitvec::prelude::*;

pub type KPKE_EncryptionKey <const k: usize> = (Vector<{k}>, [u8; 32]);
pub type KPKE_DecryptionKey <const k: usize> = Vector<{k}>;

pub type KPKE_KeyGen_Output <const k: usize> = (KPKE_EncryptionKey<{k}>, KPKE_DecryptionKey<{k}>);

pub fn key_gen<PARAMS: MlKemParams>() -> KPKE_KeyGen_Output<{PARAMS::k}> where
    [(); PARAMS::k]: ,
    [(); PARAMS::eta_2]: ,
    [(); 64 * PARAMS::eta_1]: ,
{
    let d = crypt::random_bytes::<32>();
    let (rho, sigma) = crypt::G::<32>(&d);

    let mut n = 0;

    // Our public key, (the bad bases)
    let mut A: Matrix<{PARAMS::k}> = Matrix::new(RingRepresentation::NTT);

    for i in 0..PARAMS::k {
        for j in 0..PARAMS::k {
            A.data[i][j] = sample::sample_ntt(crypt::XOF::new(&rho, i as u8, j as u8)) // XOF stream is instantied here for each index of the matrix
        }
    }

    // Our secret key
    let mut s = Vector::new(RingRepresentation::Degree255); //This is ugly, maybe use an iterator to make the polynomials, then collect them into a vector
    for i in 0..PARAMS::k {
        s.data[i] = sample::sample_poly_cbd::<{PARAMS::eta_1}>(
            crypt::prf::<{PARAMS::eta_1}>(&sigma, n)
        );
        n += 1;
    }


    // Our error vector
    let mut e = Vector::new(RingRepresentation::Degree255);
    for i in 0..PARAMS::k {
        e.data[i] = sample::sample_poly_cbd::<{PARAMS::eta_1}>(
            crypt::prf::<{PARAMS::eta_1}>(&sigma, n)
        );
        n += 1;
    }

    // NTT both
    s.ntt();
    e.ntt();

    let t = A.right_vector_multiply(&s).add(&e);

    ((t, rho), s)
}

pub type Cyphertext<const k: usize> = (Vector<{k}>, Ring);

pub fn encrypt<PARAMS: MlKemParams>(ek_pke: KPKE_EncryptionKey<{PARAMS::k}>, m: [u8; 32], rand: [u8; 32]) -> Cyphertext<{PARAMS::k}> where
    [(); PARAMS::k]: ,
    [(); 64 * PARAMS::eta_1]: ,
    [(); 64 * PARAMS::eta_2]: ,
    [(); 384 * PARAMS::k + 32]:
{
    let mut n = 0;

    let (t, rho) = ek_pke; // rho is the seed for A, the matrix, t comes from KeyGen's computation with their secret

    // Recreate the matrix A
    let mut A: Matrix<{PARAMS::k}> = Matrix::new(RingRepresentation::NTT);
    for i in 0..PARAMS::k {
        for j in 0..PARAMS::k {
            A.data[i][j] = sample::sample_ntt(crypt::XOF::new(&rho, i as u8, j as u8));
        }
    }

    // Encrpytor's Secret (Equivalent of S in key_gen)
    let mut r: Vector<{PARAMS::k}> = Vector::new(RingRepresentation::Degree255);
    for i in 0..PARAMS::k {
        r.data[i] = sample::sample_poly_cbd::<{PARAMS::eta_1}>(
            crypt::prf::<{PARAMS::eta_1}>(&rand, n)
        );
        n += 1;
    }

    // Error vector to be added to R^T * A
    let mut e_1: Vector<{PARAMS::k}> = Vector::new(RingRepresentation::Degree255);
    for i in 0..PARAMS::k {
        e_1.data[i] = sample::sample_poly_cbd::<{PARAMS::eta_2}>(
            crypt::prf::<{PARAMS::eta_2}>(&rand, n)
        );
        n += 1;
    }

    // Error vector to be added to the shared key V (R^T * t) 
    let e_2 = sample::sample_poly_cbd::<{PARAMS::eta_2}>(
        crypt::prf::<{PARAMS::eta_2}>(&rand, n)
    );

    r.ntt();

    // u is the encryptors computation with A and their secret, but this one is left-multiplied
    let u = (A.left_vector_multiply(&r)).inverse_ntt().add(&e_1);

    let mu = Decompress::<1>(&ByteDecode::<1>(m.view_bits::<BitOrder>())); // Our message is now a ring with elements 0 or q/2

    // v is our shared secret, notice for both parties its approximately rAs.
    let v = *(r.inner_product(&t)).inverse_ntt().add(&e_2).add(&mu);

    (u, v)
}

pub fn decrypt<PARAMS: MlKemParams>(
    dk_pke: [u8; 384 * PARAMS::k],
    c: [u8; 32*(PARAMS::d_u * PARAMS::k + PARAMS::d_v)]
) -> [u8; 32] {
    todo!();
}