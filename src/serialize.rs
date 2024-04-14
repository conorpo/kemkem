use sha3::digest::typenum::bit;
use sha3::digest::typenum::Bit;

// Handles Compression and Byte Serialization
use crate::crypt::*;
use crate::kpke::*;
use crate::params::*;
use crate::ring::*;
use bitvec::prelude::*;
use bitvec::access::*;

pub type BitOrder = Msb0;

pub fn ByteEncode<const d: usize>(F: &Ring, bitvec_slice: &mut BitSlice<BitSafeU8, BitOrder>) {
    for (slot, ele) in bitvec_slice.chunks_mut(d).zip(F.data.iter()) {
        slot.store_be(*ele);
    }
}

pub fn ByteDecode<const d: usize>(bitvec_slice: &BitSlice<u8, BitOrder>) -> Ring {
    let mut F = Ring::new(RingRepresentation::Degree255);
    for (slot, ele) in bitvec_slice.chunks(d).zip(F.data.iter_mut()) {
        *ele = slot.load_be();
    }
    F
}
pub trait MlKemSerialize<PARAMS: MlKemParams> {
    fn serialize(&self) -> BitVec<u8, BitOrder>;
}

const DESERIALIZE_ERROR: &'static str = "Failed to deserialize";
pub trait MlKemDeserialize<PARAMS: MlKemParams> {
    fn deserialize(bitvec: &BitVec<u8, BitOrder>) -> Self;
}

impl<PARAMS: MlKemParams> MlKemSerialize<PARAMS> for KpkeEncryptionKey<{PARAMS::K}> {
    fn serialize(&self) -> BitVec<u8, BitOrder> {
        let mut bitvec = bitvec![u8, BitOrder; 0; PARAMS::K * 256 * 12 + 32];
        for (i, chunk) in bitvec.chunks_mut(256 * 12).enumerate() {
            ByteEncode::<12>(&self.0.data[i], chunk);
        }

        // Serialize rho into last 32 bytes
        bitvec[256 * 12 * PARAMS::K..].chunks_mut(8).enumerate().for_each(|(i, chunk)| {
            chunk.store_be(self.1[i]);
        });

        bitvec
    }
}

impl<PARAMS: MlKemParams> MlKemDeserialize<PARAMS> for KpkeEncryptionKey<{PARAMS::K}> {
    fn deserialize(bitvec: &BitVec<u8, BitOrder>) -> Self {
        let (t_slice, rho_slice) = bitvec.split_at(256 * 12 * PARAMS::K);

        let mut t = Vector::new(RingRepresentation::Degree255);
        for (k, ring) in t_slice.chunks(256 * 12).enumerate() {
            t.data[k] = ByteDecode::<12>(ring);
        }

        let mut rho = [0u8; 32];
        for (i, chunk) in rho_slice.chunks(8).enumerate() {
            rho[i] = chunk.load_be();
        }

        (t, rho)
    }
}

impl<PARAMS: MlKemParams> MlKemSerialize<PARAMS> for Cyphertext<{PARAMS::K}> where 
    [(); 32*(PARAMS::D_U * PARAMS::K + PARAMS::D_V)]:
{
    fn serialize(&self) -> BitVec<u8, BitOrder> {
        let mut bitvec = bitvec![u8, BitOrder; 0; 32*(PARAMS::D_U * PARAMS::K + PARAMS::D_V)];
        
        for (k, ring) in bitvec.chunks_mut(PARAMS::D_U * 256).enumerate() {
            ByteEncode::<{PARAMS::D_U}>(&self.0.data[k], ring);
        }

        for (_, ring) in bitvec.chunks_mut(PARAMS::D_V * 256).enumerate() {
            ByteEncode::<{PARAMS::D_V}>(&self.1, ring);
        }

        bitvec
    }
}

// Serialize is only every used on Message Ring, bad practice but this defines serializaiton of that specific ring
impl<PARAMS: MlKemParams> MlKemSerialize<PARAMS> for Ring {
    fn serialize(&self) -> BitVec<u8, BitOrder> {
        let mut bitvec = bitvec![u8, BitOrder; 32];

        ByteEncode::<1>(self, bitvec.chunks_mut(256).next().unwrap());

        bitvec
    }

    
}