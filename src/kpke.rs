use crate::crypt;
use crate::params::*;
use crate::polynomial::*;
use crate::sample::*;

pub fn KeyGen<const k: usize>() -> (Matrix<Polynomial>, Vector<Polynomial>, Vector<Polynomial>) {
    let d = crypt::random_bytes(32);
    let (rho, sigma) = crypt::G(d);

    let mut N = 0;

    // Our public key, the bad bases
    let mut A: Matrix<k> = Matrix::new(PolynomialType::NTT);

    for i in 0..k {
        for j in 0..k {
            A.data[i][j] = SampleNTT(XOF::new(rho, i, j)) // XOF stream is instantied here for each index of the matrix
        }
    }

    // Our secret key
    let mut S: Vec<k> = Vec::new(PolynomialType::Ring);
    for i in 0..params.k {
        S.push( match params.eta_1 {
            2 => SamplePolyCBD_2(PRF_2(sigma, N)),
            3 => SamplePolyCBD_3(PRF_3(sigma, N)),
            _ => panic!("Invalid eta_1")
        });
        N += 1;
    }


    // Our error vector
    let mut E: Vec<k> = Vec::new(PolynomialType::Ring);

    for i in 0..params.k {
        E.push( match params.eta_2 {
            2 => SamplePolyCBD_2(PRF_2(sigma, N)),
            3 => SamplePolyCBD_3(PRF_3(sigma, N)),
            _ => panic!("Invalid eta_2")
        });
        N += 1;
    }

    // NTT both
    let S = S.NTT();
    let E = E.NTT();

    let T = A.vector_multiply(&S).add(&E);

    todo!() // byte encode (t)||p and s   
    //(t||p) is the encapsulation key, s is the secret (decapsulation) key
}