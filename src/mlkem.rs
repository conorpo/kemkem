use bitvec::view::BitView;

use crate::debug_values::DebugValues;
use crate::kpke::KpkeDecryptionKey;
use crate::kpke::KpkeEncryptionKey;
use crate::kpke::KpkeKeyGenOutput;
use crate::params::*;
use crate::crypt;
use crate::kpke;
use crate::ring::Compressed;
use crate::serialize::*;
use crate::ring::Ring;

use bitvec::prelude::Msb0;


pub type MlkemEncapsulationKey<const k: usize> = KpkeEncryptionKey<k>;
pub type MlkemDecapsulationKey<const k: usize> = (KpkeDecryptionKey<k>, KpkeEncryptionKey<k>, [u8; 32], [u8; 32]);

// ML-KEM.KeyGen
pub fn key_gen<PARAMS: MlKemParams + DebugValues<PARAMS>> () -> (MlkemEncapsulationKey<{PARAMS::K}>, MlkemDecapsulationKey<{PARAMS::K}>) where
    [(); 768 * PARAMS::K + 96]: ,
    [(); PARAMS::ETA_1]: ,
    [(); PARAMS::ETA_2]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 384 * PARAMS::K + 32]: ,
    [(); 32 * (PARAMS::D_U * PARAMS::K + PARAMS::D_V)]: ,
{
    let z = match cfg!(debug_assertions) {
        true => PARAMS::KEYGEN_DEBUG.z,
        false => crypt::random_bytes::<32>()
    };

    debug!("z: {}", {
        z.iter()
         .map(|byte| format!("{:02X}", byte))
         .collect::<String>()
    });

    //Encryption key, Decryption key
    let (ek, dk) : KpkeKeyGenOutput<{PARAMS::K}> = kpke::key_gen::<PARAMS>();
    
    // Encapsulation key is the encryption key
    let encapsulation_key: MlkemEncapsulationKey<{PARAMS::K}> = ek;

    let hash = crypt::H(encapsulation_key.serialize().into_vec());

    // Fujisaki-Okamoto transformation, turn decryption key into decapsulation
    let decapsulation_key: MlkemDecapsulationKey<{PARAMS::K}> = (dk, encapsulation_key.clone(), hash, z);

    debug!("ek: {}\ndk: {}", {
        encapsulation_key.serialize().as_raw_slice().iter()
          .map(|byte| format!("{:02X}", byte))
          .collect::<String>()
    }, {
        decapsulation_key.serialize().as_raw_slice().iter()
          .map(|byte| format!("{:02X}", byte))
          .collect::<String>()
    });

    debug_assert_eq!(encapsulation_key.serialize(), PARAMS::KEYGEN_DEBUG.ek.view_bits::<Msb0>().to_bitvec());
    debug_assert_eq!(decapsulation_key.serialize(), PARAMS::KEYGEN_DEBUG.dk.view_bits::<Msb0>().to_bitvec());
    
    (encapsulation_key, decapsulation_key)
}

// ML-KEM.Encaps
                                                                               //  Shared Key
pub fn encaps<PARAMS: MlKemParams + DebugValues<PARAMS>>(ek_mlkem: MlkemEncapsulationKey<{PARAMS::K}>) -> ([u8;32], kpke::Cyphertext<{PARAMS::K}, {PARAMS::D_U}, {PARAMS::D_V}>) where
    [(); 768 * PARAMS::K + 96]: ,
    [(); PARAMS::K]: ,
    [(); 384 * PARAMS::K + 32]: ,
    [(); 32*(PARAMS::D_U * PARAMS::K + PARAMS::D_V)]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 64 * PARAMS::ETA_2]: 
{
    let m = match cfg!(debug_assertions) {
        true => PARAMS::ENCAPS_DEBUG.m,
        false => crypt::random_bytes::<32>()
    };

    let ek_hash = crypt::H(ek_mlkem.serialize().into_vec());

    let mut combined = [0u8; 64];

    combined[..32].copy_from_slice(&m);
    combined[32..].copy_from_slice(&ek_hash);

    let (key, r) = crypt::G::<64>(&combined);

    debug!("K: {}", {
        key.iter()
           .map(|byte| format!("{:02X}", byte))
           .collect::<String>()
    });
    debug!("r: {}", {
        r.iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<String>()
    });

    debug_assert_eq!(key, PARAMS::ENCAPS_DEBUG.key);
    debug_assert_eq!(r, PARAMS::ENCAPS_DEBUG.r);

    
    let m: Compressed<1, Ring> = Compressed::<1, Ring>::deserialize(&m.view_bits::<BitOrder>().to_bitvec());

    //println!("encaps_m: {:?}", m);

    // Encrypt the encapsulation key
    let c = kpke::encrypt::<PARAMS>(ek_mlkem, m, r);

    debug!("c: {}", {
        c.serialize().as_raw_slice().iter()
         .map(|byte| format!("{:02X}", byte))
         .collect::<String>()
    });

    debug_assert_eq!(c.serialize(), PARAMS::ENCAPS_DEBUG.c.view_bits::<BitOrder>().to_bitvec());


    (key, c)
}

// ML-KEM.Decaps
pub fn decaps<PARAMS: MlKemParams>(c: kpke::Cyphertext<{PARAMS::K}, {PARAMS::D_U}, {PARAMS::D_V}>, dk_mlkem: MlkemDecapsulationKey<{PARAMS::K}>) -> [u8; 32] where
    [(); PARAMS::K]: ,
    [(); 384 * PARAMS::K + 32]: ,
    [(); 32*(PARAMS::D_U * PARAMS::K + PARAMS::D_V)]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 64 * PARAMS::ETA_2]: ,
    [(); 384 * PARAMS::K + 32]:
{
    let (dk, ek, hash, z) = dk_mlkem;

    let m = kpke::decrypt::<PARAMS>(dk, c.clone());

    let mut combined = [0u8; 64];

    combined[..32].copy_from_slice(m.serialize().as_raw_slice());
    combined[32..].copy_from_slice(&hash);

    let (key, rand) = crypt::G::<64>(&combined);

    let key_reject = crypt::J([&z, c.serialize().as_raw_slice()].concat());

    let c_prime = kpke::encrypt::<PARAMS>(ek, m, rand); // Should be same as encaps

    match (c.0 == c_prime.0) && (c.1 == c_prime.1) {
        true => {
            print!("Key accepted");
            key
        },
        false => {
            print!("Key rejected");
            key_reject
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::*;

    #[test]
    fn test_keygen() {
        let (ek, dk) = key_gen::<MlKem1024>();
    }

    fn test_encaps() {
        let ek = MlKem1024::KEYGEN_DEBUG.ek.view_bits::<BitOrder>().to_bitvec();

        let (key, c) = encaps::<MlKem1024>(MlkemEncapsulationKey::<4>::deserialize(&ek));
    }
}