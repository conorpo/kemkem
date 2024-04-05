use crate::crypt;
use crate::params::*;
use crate::polynomial::*;
use crate::sample::*;

pub fn KeyGen(params: MlKemParams) -> (Vec<u8>, Vec<u8>) {
    let d = crypt::random_bytes(32);
    let (rho, sigma) = crypt::G(d);

    let mut N = 0;

    // Our public key, the bad bases
    let mut A: Vec<Vec<Polynomial>> = Vec::new();

    for i in 0..params.k {
        let mut row = Vec::new();
        for j in 0..params.k {
            //row.push(SampleNTT(crypt::XOF))
        }
    
    }

    let A: Matrix<Polynomial> = Matrix {
        data: A
    };

    // Our secret key
    let mut S: Vec<Polynomial> = Vec::new();
    for i in 0..params.k {
        S.push( match params.eta_1 {
            2 => SamplePolyCBD_2(PRF_2(sigma, N)),
            3 => SamplePolyCBD_3(PRF_3(sigma, N)),
            _ => panic!("Invalid eta_1")
        });
        N += 1;
    }
    
    let S: Vector<Polynomial> = Vector {
        data: S
    };


    // Our error vector
    let mut E: Vec<Polynomial> = Vec::new();

    for i in 0..params.k {
        E.push( match params.eta_2 {
            2 => SamplePolyCBD_2(PRF_2(sigma, N)),
            3 => SamplePolyCBD_3(PRF_3(sigma, N)),
            _ => panic!("Invalid eta_2")
        });
        N += 1;
    }

    let E: Vector<Polynomial> = Vector {
        data: E
    };

    // NTT both
    let S = S.NTT();
    let E = E.NTT();

    let T = A.vector_multiply(&S).add(&E);
    
    

    reutrn 
    
}