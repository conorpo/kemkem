use crate::params::*;
use crate::crypt;
use crate::kpke;

struct DecapsulationKey<const k: usize> where 
    [u8; 384*k + 32]:  // Generic Const Expr garunteed to make valid array
{
    dk: [u8; 32],
    ek: [u8; 384*k + 32],
    ek_hash: [u8; 32],
    z: [u8; 32] //Implicit Rejection Term
}

// ML-KEM.KeyGen
pub fn key_gen<PARAMS: MlKemParams> () -> ([u8; 384*PARAMS::k + 32], DecapsulationKey::<{PARAMS::k}>) where 
    [(); PARAMS::eta_1]: ,
    [(); PARAMS::eta_2]: ,
    [(); 64 * PARAMS::eta_1]: ,
    [(); 384 * PARAMS::k + 32]:
{
    let z = crypt::random_bytes::<32>();

    //Encryption key, Decryption key
    let (ek, dk) = kpke::key_gen::<PARAMS>();

    // Fujisaki-Okamoto transformation, turn decryption key into decapsulation
    let decap = DecapsulationKey {
        dk: dk,
        ek: ek,
        ek_hash: crypt::H(ek),
        z: z
    };

    // Encapsulation key is just the encryption key
    (ek, decap)
}