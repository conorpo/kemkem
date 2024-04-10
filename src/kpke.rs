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

    let T = A.vector_multiply(&S).add(&E);


    todo!();
    //let mut ek_pke_t = [Encoded<12>; k];

    //(t||p) is the encapsulation key, s is the secret (decapsulation) key
}

pub fn Encrypt<const k: usize>(ek_pke, m: [u8; 32], r: [u8; 32]) -> ([u8; 32], [u8; 128]) {
    let (rho, sigma) = crypt::G(ek_pke);

    let mut N = 0;

    let mut u: Vec<k> = Vec::new(PolynomialType::Ring);
    for i in 0..params.k {
        u.push( match params.eta_1 {
            2 => SamplePolyCBD_2(PRF_2(sigma, N)),
            3 => SamplePolyCBD_3(PRF_3(sigma, N)),
            _ => panic!("Invalid eta_1")
        });
        N += 1;
    }

    let mut v: Vec<k> = Vec::new(PolynomialType::Ring);
    for i in 0..params.k {
        v.push( match params.eta_2 {
            2 => SamplePolyCBD_2(PRF_2(sigma, N)),
            3 => SamplePolyCBD_3(PRF_3(sigma, N)),
            _ => panic!("Invalid eta_2")
        });
        N += 1;
    }

    let u = u.NTT();
    let v = v.NTT();

    let c = ek_pke.vector_multiply(&u).add(&v);

    let t = crypt::H(c);
    let k = crypt::J(c);

    let c = crypt::prf_2(t, 0);

    (k, c)
}