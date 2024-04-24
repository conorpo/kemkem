use bitvec::view::BitView;

use crate::kpke;
use kpke::{
    KpkeKeyGenOutput,
    KpkeEncryptionKey,
    KpkeDecryptionKey
};

use crate::params::*;
use crate::crypt;
use crate::ring::*;
use crate::serialize::*;

pub type MlkemEncapsulationKey<const K: usize> = KpkeEncryptionKey<K>;
pub type MlkemDecapsulationKey<const K: usize> = (KpkeDecryptionKey<K>, KpkeEncryptionKey<K>, [u8; 32], [u8; 32]);

// ML-KEM.KeyGen
pub fn key_gen<PARAMS: MlKemParams> () -> (MlkemEncapsulationKey<{PARAMS::K}>, MlkemDecapsulationKey<{PARAMS::K}>) where
    [(); 768 * PARAMS::K + 96]: ,
    [(); PARAMS::ETA_1]: ,
    [(); PARAMS::ETA_2]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 384 * PARAMS::K + 32]: ,
    [(); 32 * (PARAMS::D_U * PARAMS::K + PARAMS::D_V)]: ,
{
    let z = crypt::random_bytes::<32>();

    //Encryption key, Decryption key
    let (ek, dk) : KpkeKeyGenOutput<{PARAMS::K}> = kpke::key_gen::<PARAMS>();
    
    // Encapsulation key is the encryption key
    let encapsulation_key: MlkemEncapsulationKey<{PARAMS::K}> = ek;

    let hash = crypt::h(&encapsulation_key.serialize().into_vec());

    // Fujisaki-Okamoto transformation, turn decryption key into decapsulation
    let decapsulation_key: MlkemDecapsulationKey<{PARAMS::K}> = (dk, encapsulation_key.clone(), hash, z);
    
    (encapsulation_key, decapsulation_key)
}

// ML-KEM.Encaps
pub type MlKemCyphertext<const K: usize, const D_U: usize, const D_V: usize> = kpke::Cyphertext<K, D_U, D_V>;
                                                                               //  Shared Key
pub fn encaps<PARAMS: MlKemParams>(ek_mlkem: MlkemEncapsulationKey<{PARAMS::K}>) -> ([u8;32], MlKemCyphertext<{PARAMS::K}, {PARAMS::D_U}, {PARAMS::D_V}>) where
    [(); 768 * PARAMS::K + 96]: ,
    [(); PARAMS::K]: ,
    [(); 384 * PARAMS::K + 32]: ,
    [(); 32*(PARAMS::D_U * PARAMS::K + PARAMS::D_V)]: ,
    [(); 64 * PARAMS::ETA_1]: ,
    [(); 64 * PARAMS::ETA_2]: 
{
    let m = crypt::random_bytes::<32>();

    let ek_hash = crypt::h(&ek_mlkem.serialize().into_vec());

    let mut combined = [0u8; 64];

    combined[..32].copy_from_slice(&m);
    combined[32..].copy_from_slice(&ek_hash);

    let (key, r) = crypt::g::<64>(&combined);
    
    let m: Compressed<1, Ring> = Compressed::<1, Ring>::deserialize(&m.view_bits::<BitOrder>().to_bitvec());

    // Encrypt the encapsulation key
    let c = kpke::encrypt::<PARAMS>(ek_mlkem, m, r);

    (key, c)
}

// ML-KEM.Decaps
pub fn decaps<PARAMS: MlKemParams>(c: MlKemCyphertext<{PARAMS::K}, {PARAMS::D_U}, {PARAMS::D_V}>, dk_mlkem: MlkemDecapsulationKey<{PARAMS::K}>) -> [u8; 32] where
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

    let (key, rand) = crypt::g::<64>(&combined);

    let key_reject = crypt::j([&z, c.serialize().as_raw_slice()].concat());

    let c_prime = kpke::encrypt::<PARAMS>(ek, m, rand); // Should be same as encaps

    match (c.0 == c_prime.0) && (c.1 == c_prime.1) {
        true => key,
        false => key_reject
    }
}


mod tests {
    
    #[test]
    fn test_mlkem<>(){
        use super::*;
        type PARAMS = MlKem512;

        //ML-KEM.KeyGen
        let (ek, dk) = key_gen::<PARAMS>();

        let ek = ek.serialize();
        let dk = dk.serialize();

        //ML-KEM.Encaps
        let ek = MlkemEncapsulationKey::<{PARAMS::K}>::deserialize(&ek);
        
        let (key, c) = encaps::<PARAMS>(ek);

        let c = c.serialize();

        //ML-KEM.Decaps
        let dk = MlkemDecapsulationKey::<{PARAMS::K}>::deserialize(&dk);
        let c = MlKemCyphertext::<{PARAMS::K}, {PARAMS::D_U}, {PARAMS::D_V}>::deserialize(&c);

        let key_prime = decaps::<PARAMS>(c, dk);

        assert_eq!(key, key_prime);
    }
}