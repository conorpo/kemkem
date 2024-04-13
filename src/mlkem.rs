use crate::kpke::KPKE_DecryptionKey;
use crate::kpke::KPKE_EncryptionKey;
use crate::kpke::KPKE_KeyGen_Output;
use crate::params::*;
use crate::crypt;
use crate::kpke;
use crate::random_bytes;
use crate::serialize::MlKemSerialize;

type MLKEM_EncapsulationKey<const k: usize> = KPKE_EncryptionKey<k>;
type MLKEM_DecapsulationKey<const k: usize> = (KPKE_DecryptionKey<k>, KPKE_EncryptionKey<k>, [u8; 32], [u8; 32]);

// ML-KEM.KeyGen
pub fn key_gen<PARAMS: MlKemParams> () -> (MLKEM_EncapsulationKey<{PARAMS::k}>, MLKEM_DecapsulationKey<{PARAMS::k}>) where
    [(); PARAMS::eta_1]: ,
    [(); PARAMS::eta_2]: ,
    [(); 64 * PARAMS::eta_1]: ,
    [(); 384 * PARAMS::k + 32]:
{
    let z = crypt::random_bytes::<32>();

    //Encryption key, Decryption key
    let (ek, dk) : KPKE_KeyGen_Output<{PARAMS::k}> = kpke::key_gen::<PARAMS>();
    
    // Encapsulation key is the encryption key
    let encapsulation_key: MLKEM_EncapsulationKey<{PARAMS::k}> = ek;

    let hash = crypt::H(encapsulation_key.serialize().into_vec());

    // Fujisaki-Okamoto transformation, turn decryption key into decapsulation
    let decapsulation_key: MLKEM_DecapsulationKey<{PARAMS::k}> = (dk, encapsulation_key.clone(), hash, z);

    (encapsulation_key, decapsulation_key)
}

// ML-KEM.Encaps
                                                                                //  Shared Key
pub fn encaps<PARAMS: MlKemParams>(ek_mlkem: MLKEM_EncapsulationKey<{PARAMS::k}>) -> ([u8;32], kpke::Cyphertext<{PARAMS::k}>) where
    [(); PARAMS::k]: ,
    [(); 384 * PARAMS::k + 32]: ,
    [(); 32*(PARAMS::d_u * PARAMS::k + PARAMS::d_v)]: ,
    [(); 64 * PARAMS::eta_1]: ,
    [(); 64 * PARAMS::eta_2]: 
{
    let m = random_bytes::<32>();

    let ek_hash = crypt::H(ek_mlkem.serialize().into_vec());

    let mut combined = [0u8; 64];

    combined[..32].copy_from_slice(&m);
    combined[32..].copy_from_slice(&ek_hash);

    let (K, rand) = crypt::G::<64>(&combined);

    // Encrypt the encapsulation key
    let c = kpke::encrypt::<PARAMS>(ek_mlkem, m, rand);

    (K, c)
}

// ML-KEM.Decaps
pub fn decaps<PARAMS: MlKemParams>(c: kpke::Cyphertext<{PARAMS::k}>, dk_mlkem: MLKEM_DecapsulationKey<{PARAMS::k}>) -> [u8; 32] where
    [(); PARAMS::k]: ,
    [(); 384 * PARAMS::k + 32]: ,
    [(); 32*(PARAMS::d_u * PARAMS::k + PARAMS::d_v)]: ,
    [(); 64 * PARAMS::eta_1]: ,
    [(); 64 * PARAMS::eta_2]: 
{
    let (dk, ek, hash, z) = dk_mlkem;

    //let m = kpke::decrypt::<PARAMS>(dk, c);

    let mut combined = [0u8; 64];

    combined[..32].copy_from_slice(&c.1.serialize().into_vec());
    combined[32..].copy_from_slice(&hash);

    let (K, rand) = crypt::G::<64>(&combined);

    todo!();
    let k_reject = crypt::J([&z, c.serialize() ])

    let c_prime = kpke::encrypt::<PARAMS>(ek, m, rand); // Should be same as encaps
    
    todo!("Serialize");
    match c == c_prime {
        true => K,
        false => k_reject
    }
}