use crate::crypt;
use crate::params::*;
use crate::polynomial::*;
use crate::sample;
use crate::bits::*;


pub fn key_gen<params: MlKemParams>() -> (([Encoded<12>; params::k], [u8; 32]), Encoded<12>) where 
    [(); params::k]: ,
    [(); params::eta_2]: ,
    [(); 64 * params::eta_1]: ,
{
    let d = crypt::random_bytes::<32>();
    let (rho, sigma) = crypt::G::<32>(d);

    let mut n = 0;

    // Our public key, (the bad bases)
    let mut A: Matrix<{params::k}> = Matrix::new(RingRepresentation::NTT);

    for i in 0..params::k {
        for j in 0..params::k {
            A.data[i][j] = sample::sample_ntt(crypt::XOF::new(rho, i as u8, j as u8)) // XOF stream is instantied here for each index of the matrix
        }
    }

    // Our secret key
    let mut S = Vector::new(RingRepresentation::Degree255); //This is ugly, maybe use an iterator to make the polynomials, then collect them into a vector
    for i in 0..params::k {
        S.data[i] = sample::sample_poly_cbd::<{params::eta_1}>(
            crypt::prf::<{params::eta_1}>(sigma, n)
        );
        n += 1;
    }


    // Our error vector
    let mut E = Vector::new(RingRepresentation::Degree255);
    for i in 0..params::k {
        E.data[i] = sample::sample_poly_cbd::<{params::eta_1}>(
            crypt::prf::<{params::eta_1}>(sigma, n)
        );
        n += 1;
    }

    // NTT both
    let S = S.NTT();
    let E = E.NTT();

    let T = A.right_vector_multiply(&S).add(&E);


    todo!();
    //let mut ek_pke_t = [Encoded<12>; k];

    //(t||p) is the encapsulation key, s is the secret (decapsulation) key
}

pub fn encrypt<params: MlKemParams>(ek_pke: [u8; 384*params::k+32], m: [u8; 32], r: [u8; 32]) -> [u8; 32*(params::d_u * params::k + params::d_v)] where
    [(); params::k]: ,
    [(); 64 * params::eta_1]: ,
    [(); 64 * params::eta_2]: ,
    [(); 384 * params::k + 32]:
{
    let mut n = 0;

    let (t, rho) = ek_pke.split_at(384*params::k); // rho is the seed for A, the matrix, t comes from KeyGen's computation with their secret

    // Recreate the matrix A
    let mut A: Matrix<{params::k}> = Matrix::new(RingRepresentation::NTT);
    for i in 0..params::k {
        for j in 0..params::k {
            A.data[i][j] = sample::sample_ntt(crypt::XOF::new(rho, i as u8, j as u8));
        }
    }

    // Encrpytor's Secret (Equivalent of S in key_gen)
    let mut r: Vector<{params::k}> = Vector::new(RingRepresentation::Degree255);
    for i in 0..params::k {
        r.data[i] = sample::sample_poly_cbd::<{params::eta_1}>(
            crypt::prf::<{params::eta_1}>(r, n)
        );
        n += 1;
    }

    // Error vector to be added to R^T * A
    let mut e_1: Vector<{params::k}> = Vector::new(RingRepresentation::Degree255);
    for i in 0..params::k {
        e_1.data[i] = sample::sample_poly_cbd::<{params::eta_2}>(
            crypt::prf::<{params::eta_2}>(r, n)
        );
        n += 1;
    }

    // Error vector to be added to the shared key V (R^T * t) 
    let e_2 = sample::sample_poly_cbd::<{params::eta_2}>(
        crypt::prf::<{params::eta_2}>(r, n)
    );

    r.ntt();

    // u is the encryptors computation with A and their secret, but this one is left-multiplied
    let u = (A.left_vector_multiply(&r)).inverse_ntt().add(&e_1);
    
    todo!();
    let Mu;

    // v is our shared secret, notice for both parties its approximately rAs.
    let v = (r.inner_product(&t)).inverse_ntt().add(&e_2);

    let t = crypt::H(c);
    let k = crypt::J(c);

    let c = crypt::prf_2(t, 0);

    (k, c)
}

pub fn decrypt<params: MlKemParams>(
    dk_pke: [u8; 384 * params::k],
    c: [u8; 32*(params::d_u * params::k + params::d_v)]
) -> [u8; 32] {
    todo!();
}