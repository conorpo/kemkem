use crate::params::*;
use crate::crypt;
use crate::kpke;

struct DecapsulationKey<const k: usize> where 
    [u8; 384*k + 32]:  // Generic Const Expr garunteed to make valid array
{
    dk: [u8; 384*k],
    ek: [u8; 384*k + 32],
    ek_hash: [u8; 32],
    z: [u8; 32] //Implicit Rejection Term
}

// ML-KEM.KeyGen
pub fn key_gen<PARAMS: MlKemParams> () -> ([u8; 384*PARAMS::k + 32], [u8; 798*PARAMS::k + 96]) where 
    [(); PARAMS::eta_1]: ,
    [(); PARAMS::eta_2]: ,
    [(); 64 * PARAMS::eta_1]: ,
    [(); 384 * PARAMS::k + 32]:
{
    let z = crypt::random_bytes::<32>();

    //Encryption key, Decryption key
    let (ek, dk: []) = kpke::key_gen::<PARAMS>();

    let encapsulation_key = ek;

    // Fujisaki-Okamoto transformation, turn decryption key into decapsulation
    let decapsulation_key = DecapsulationKey<PARAMS::k> {
        dk: dk,
        ek: ek,
        ek_hash: crypt::H(ek),
        z: z
    };
    // Encapsulation key is just the encryption key
    (ek, decap)
}

struct Cyphertext<const k: usize, const d_u: usize, const d_v: usize> where 
    [u8; 32*(d_u*k + d_v)]:  // Generic Const Expr garunteed to make valid array
{
    c: [u8; 32*(d_u*k + d_v)]
}

// ML-KEM.Decaps
pub fn decaps<PARAMS: MlKemParams>(c: Cyphertext<{PARAMS::k}, {PARAMS::d_u}, {PARAMS::d_v}>, ek: [u8; 384*PARAMS::k + 32]) -> [u8; 32] where
    [(); PARAMS::k]: ,
    [(); 384 * PARAMS::k + 32]: ,
    [(); 32*(PARAMS::d_u * PARAMS::k + PARAMS::d_v)]:
{
    let ek_hash = crypt::H(ek);

    // Check if the encapsulation key is valid
    if ek_hash != crypt::H(ek) {
        return [0u8; 32*PARAMS::d_u*PARAMS::k];
    }

    // Decrypt the encapsulation key
    let dk = crypt::J(ek);

    // Decrypt the ciphertext
    let m = kpke::decrypt::<PARAMS>(dk, c);

    m
}