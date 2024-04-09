use crate::crypt;
use crate::params::*;
use crate::polynomial::*;
use crate::sample::*;
use crate::bits::*;


pub fn key_gen<const k: usize>() -> (([Encoded<12>; k], [u8; 32]), Encoded<12>){
    let d = crypt::random_bytes(32);
    let (rho, sigma) = crypt::G(d);

    let mut n = 0;

    // Our public key, the bad bases
    let mut A: Matrix<k> = Matrix::new(PolynomialType::NTT);

    for i in 0..k {
        for j in 0..k {
            A.data[i][j] = SampleNTT(XOF::new(rho, i, j)) // XOF stream is instantied here for each index of the matrix
        }
    }

    // Our secret key
    let mut S = Vector::new(PolynomialType::Ring); //This is ugly, maybe use an iterator to make the polynomials, then collect them into a vector
    for i in 0..k {
        S[i] =  match params.eta_1 {
            2 => SamplePolyCBD_2(PRF_2(sigma, n)),
            3 => SamplePolyCBD_3(PRF_3(sigma, n)),
            _ => panic!("Invalid eta_1")
        };w
        n += 1;
    }


    // Our error vector
    let mut E = Vector::new(PolynomialType::Ring);
    for i in 0..k {
        E. = match params.eta_2 {
            2 => SamplePolyCBD_2(PRF_2(sigma, n)),
            3 => SamplePolyCBD_3(PRF_3(sigma, n)),
            _ => panic!("Invalid eta_2")
        };
        n += 1;
    }

    // NTT both
    let S = S.NTT();
    let E = E.NTT();

    let T = A.vector_multiply(&S).add(&E);

    let mut ek_pke_t = [Encoded<12>; k];

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