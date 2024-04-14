use crate::crypt;
use crate::params::*;
use crate::ring::*;
use crate::sample;

pub type KpkeEncryptionKey <const k: usize> = (Vector<{k}>, [u8; 32]);
pub type KpkeDecryptionKey <const k: usize> = Vector<{k}>;

pub type KpkeKeyGenOutput <const k: usize> = (KpkeEncryptionKey<{k}>, KpkeDecryptionKey<{k}>);

pub fn key_gen<PARAMS: MlKemParams>() -> KpkeKeyGenOutput<{PARAMS::K}> where
    [(); PARAMS::K]: ,
    [(); PARAMS::ETA_2]: ,
    [(); 64 * PARAMS::ETA_1]: ,
{
    let d = crypt::random_bytes::<32>();
    let (rho, sigma) = crypt::G::<32>(&d);

    let mut n = 0;

    // Our public key, (the bad bases)
    let mut a: Matrix<{PARAMS::K}> = Matrix::new(RingRepresentation::NTT);

    for i in 0..PARAMS::K {
        for j in 0..PARAMS::K {
            a.data[i][j] = sample::sample_ntt(crypt::XOF::new(&rho, i as u8, j as u8)) // XOF stream is instantied here for each index of the matrix
        }
    }

    // Our secret key
    let mut s = Vector::new(RingRepresentation::Degree255); //This is ugly, maybe use an iterator to make the polynomials, then collect them into a vector
    for i in 0..PARAMS::K {
        s.data[i] = sample::sample_poly_cbd::<{PARAMS::ETA_1}>(
            crypt::prf::<{PARAMS::ETA_1}>(&sigma, n)
        );
        n += 1;
    }


    // Our error vector
    let mut e = Vector::new(RingRepresentation::Degree255);
    for i in 0..PARAMS::K {
        e.data[i] = sample::sample_poly_cbd::<{PARAMS::ETA_1}>(
            crypt::prf::<{PARAMS::ETA_1}>(&sigma, n)
        );
        n += 1;
    }

    // NTT both
    s.ntt();
    e.ntt();

    let t = a.right_vector_multiply(&s).add(&e);

    ((t, rho), s)
}

pub type Cyphertext<const k: usize> = (Vector<{k}>, Ring);


pub fn encrypt<PARAMS: MlKemParams>(ek_pke: KpkeEncryptionKey<{PARAMS::K}>, mut m: Ring, rand: [u8; 32]) -> Cyphertext<{PARAMS::K}> where
    [(); PARAMS::K]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 64 * PARAMS::ETA_2]: ,
    [(); 384 * PARAMS::K + 32]: ,
{
    let mut n = 0;

    let (t, rho) = ek_pke; // rho is the seed for A, the matrix, t comes from KeyGen's computation with their secret

    // Recreate the matrix A
    let mut A: Matrix<{PARAMS::K}> = Matrix::new(RingRepresentation::NTT);
    for i in 0..PARAMS::K {
        for j in 0..PARAMS::K {
            A.data[i][j] = sample::sample_ntt(crypt::XOF::new(&rho, i as u8, j as u8));
        }
    }

    // Encrpytor's Secret (Equivalent of S in key_gen)
    let mut r: Vector<{PARAMS::K}> = Vector::new(RingRepresentation::Degree255);
    for i in 0..PARAMS::K {
        r.data[i] = sample::sample_poly_cbd::<{PARAMS::ETA_1}>(
            crypt::prf::<{PARAMS::ETA_1}>(&rand, n)
        );
        n += 1;
    }

    // Error vector to be added to R^T * A
    let mut e_1: Vector<{PARAMS::K}> = Vector::new(RingRepresentation::Degree255);
    for i in 0..PARAMS::K {
        e_1.data[i] = sample::sample_poly_cbd::<{PARAMS::ETA_2}>(
            crypt::prf::<{PARAMS::ETA_2}>(&rand, n)
        );
        n += 1;
    }

    // Error vector to be added to the shared key V (R^T * t) 
    let e_2 = sample::sample_poly_cbd::<{PARAMS::ETA_2}>(
        crypt::prf::<{PARAMS::ETA_2}>(&rand, n)
    );

    r.ntt();

    // u is the encryptors computation with A and their secret, but this one is left-multiplied
    let u = *(A.left_vector_multiply(&r))
                            .inverse_ntt()
                            .add(&e_1)
                            .compress(PARAMS::D_U);

    let mu = *m.decompress(1); // Our message is now a ring with elements 0 or q/2

    // v is our shared secret, notice for both parties its approximately rAs.
    let v = *(r.inner_product(&t))
                    .inverse_ntt()
                    .add(&e_2)
                    .add(&mu)
                    .compress(PARAMS::D_V);

    (u, v)
}

pub fn decrypt<PARAMS: MlKemParams>(dk_kpe: Vector<{PARAMS::K}>, c: Cyphertext<{PARAMS::K}>) -> Ring {
    let mut c = c;  
    //Decompress Cyphertext
    let u = c.0.decompress(PARAMS::D_U); // rA + e from the encryptor
    let v = c.1.decompress(PARAMS::D_V); // rt + e + m from the encryptor

    let w = v.sub(&(dk_kpe.inner_product(&u.ntt())).inverse_ntt());

    *w.compress(1)
}