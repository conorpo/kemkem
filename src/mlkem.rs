use crate::kpke::KpkeDecryptionKey;
use crate::kpke::KpkeEncryptionKey;
use crate::kpke::KpkeKeyGenOutput;
use crate::params::*;
use crate::crypt;
use crate::kpke;
use crate::random_bytes;
use crate::serialize::*;
use crate::ring::Ring;

type MLKEM_EncapsulationKey<const k: usize> = KpkeEncryptionKey<k>;
type MLKEM_DecapsulationKey<const k: usize> = (KpkeDecryptionKey<k>, KpkeEncryptionKey<k>, [u8; 32], [u8; 32]);

// ML-KEM.KeyGen
pub fn key_gen<PARAMS: MlKemParams> () -> (MLKEM_EncapsulationKey<{PARAMS::K}>, MLKEM_DecapsulationKey<{PARAMS::K}>) where
    [(); PARAMS::ETA_1]: ,
    [(); PARAMS::ETA_2]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 384 * PARAMS::K + 32]:
{
    let z = crypt::random_bytes::<32>();

    //Encryption key, Decryption key
    let (ek, dk) : KpkeKeyGenOutput<{PARAMS::K}> = kpke::key_gen::<PARAMS>();
    
    // Encapsulation key is the encryption key
    let encapsulation_key: MLKEM_EncapsulationKey<{PARAMS::K}> = ek;

    let hash = crypt::H(encapsulation_key.serialize().into_vec());

    // Fujisaki-Okamoto transformation, turn decryption key into decapsulation
    let decapsulation_key: MLKEM_DecapsulationKey<{PARAMS::K}> = (dk, encapsulation_key.clone(), hash, z);

    (encapsulation_key, decapsulation_key)
}

// ML-KEM.Encaps
                                                                               //  Shared Key
pub fn encaps<PARAMS: MlKemParams>(ek_mlkem: MLKEM_EncapsulationKey<{PARAMS::K}>) -> ([u8;32], kpke::Cyphertext<{PARAMS::K}>) where
    [(); PARAMS::K]: ,
    [(); 384 * PARAMS::K + 32]: ,
    [(); 32*(PARAMS::D_U * PARAMS::K + PARAMS::D_V)]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 64 * PARAMS::ETA_2]: 
{
    let m = random_bytes::<32>();

    let ek_hash = crypt::H(ek_mlkem.serialize().into_vec());

    let mut combined = [0u8; 64];

    combined[..32].copy_from_slice(&m);
    combined[32..].copy_from_slice(&ek_hash);

    let (K, rand) = crypt::G::<64>(&combined);
    
    let m = Ring::deserialize(&m);

    // Encrypt the encapsulation key
    let c = kpke::encrypt::<PARAMS>(ek_mlkem, m, rand);

    (K, c)
}

// ML-KEM.Decaps
pub fn decaps<PARAMS: MlKemParams>(c: kpke::Cyphertext<{PARAMS::K}>, dk_mlkem: MLKEM_DecapsulationKey<{PARAMS::K}>) -> [u8; 32] where
    [(); PARAMS::K]: ,
    [(); 384 * PARAMS::K + 32]: ,
    [(); 32*(PARAMS::D_U * PARAMS::K + PARAMS::D_V)]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 64 * PARAMS::ETA_2]: ,
    [(); 384 * PARAMS::K + 32]:
{
    let (dk, ek, hash, z) = dk_mlkem;

    let m = kpke::decrypt::<PARAMS>(dk, c);

    let mut combined = [0u8; 64];

    combined[..32].copy_from_slice(m.serialize().as_raw_slice());
    combined[32..].copy_from_slice(&hash);

    let (K, rand) = crypt::G::<64>(&combined);

    let K_reject = crypt::J([&z, c.serialize().as_raw_slice()].concat());

    let c_prime = kpke::encrypt::<PARAMS>(ek, m, rand); // Should be same as encaps
    
    match (c.0 == c_prime.0) && (c.1 == c_prime.1) {
        true => K,
        false => K_reject
    }
}