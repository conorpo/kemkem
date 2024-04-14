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
    let s = s.ntt();
    let e = e.ntt();

    assert_eq!(s.data[0].t == RingRepresentation::NTT, true);
    assert_eq!(e.data[0].t == RingRepresentation::NTT, true);

    let mut t = a.right_vector_multiply(&s);
    t.add(&e);

    ((t, rho), s)
}

pub type Cyphertext<const k: usize, const d_u: usize, const d_v: usize> = (Compressed<{d_u}, Vector<{k}>>, Compressed<{d_v}, Ring>);

pub fn encrypt<PARAMS: MlKemParams>(ek_pke: KpkeEncryptionKey<{PARAMS::K}>, m: Compressed<1,Ring>, rand: [u8; 32]) -> Cyphertext<{PARAMS::K}, {PARAMS::D_U}, {PARAMS::D_V}> where
    [(); PARAMS::K]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 64 * PARAMS::ETA_2]: ,
    [(); 384 * PARAMS::K + 32]: ,
    [(); {PARAMS::D_U}]: ,
{
    let mut n = 0;

    let (t, rho) = ek_pke; // rho is the seed for A, the matrix, t comes from KeyGen's computation with their secret

    // Recreate the matrix A
    let mut a: Matrix<{PARAMS::K}> = Matrix::new(RingRepresentation::NTT);
    for i in 0..PARAMS::K {
        for j in 0..PARAMS::K {
            a.data[i][j] = sample::sample_ntt(crypt::XOF::new(&rho, i as u8, j as u8));
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

    let r = r.ntt();

    // u is the encryptors computation with A and their secret, but this one is left-multiplied
    let mut u = a.left_vector_multiply(&r).inverse_ntt();
    u.add(&e_1);

    let u_compressed = Compressed::<{PARAMS::D_U}, Vector<{PARAMS::K}>>::compress(u);

    let m = m.decompress();

    // v is our shared secret, notice for both parties its approximately rAs.
    let mut v_ntt = r.inner_product(t);
    v_ntt.inverse_ntt().add(&e_2).add(&m);

    let mut v = v_ntt;
    let v_compressed = Compressed::<{PARAMS::D_V}, Ring>::compress(v);

    (u_compressed, v_compressed)
}

pub fn decrypt<PARAMS: MlKemParams>(dk_kpe: Vector<{PARAMS::K}>, c: Cyphertext<{PARAMS::K}, {PARAMS::D_U}, {PARAMS::D_V}>) -> Compressed<1, Ring> {
    //Decompress Cyphertext
    let u_compressed = c.0; // rA + e from the encryptor
    let v_compressed = c.1; // rt + e + m from the encryptor

    let u = u_compressed.decompress();

    let mut v = v_compressed.decompress();
    v.sub(&dk_kpe.inner_product(u.ntt()).inverse_ntt());

    Compressed::<1, Ring>::compress(v)
}