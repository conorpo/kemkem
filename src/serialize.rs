use sha3::digest::typenum::bit;
use sha3::digest::typenum::Bit;

// Handles Compression and Byte Serialization
use crate::crypt::*;
use crate::kpke::*;
use crate::params::*;
use crate::ring::*;
use crate::mlkem::*;
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
pub trait MlKemSerialize {
    fn serialize(&self) -> BitVec<u8, BitOrder>;
}

const DESERIALIZE_ERROR: &'static str = "Failed to deserialize";
pub trait MlKemDeserialize {
    fn deserialize(bitvec: &BitVec<u8, BitOrder>) -> Self;
}


impl<const K: usize> MlKemSerialize for MlkemEncapsulationKey<{K}> {
    fn serialize(&self) -> BitVec<u8, BitOrder> {
        let mut bitvec = bitvec![u8, BitOrder; 0; 8 * (384 * K + 32)];

        let (t_slice, rho_slice) = bitvec.split_at_mut(384 * K * 8);

        for (i, chunk) in t_slice.chunks_mut(384 * 8).enumerate() {
            ByteEncode::<12>(&self.0.data[i], chunk);
        }

        // Serialize rho into last 32 bytes
        for (i, byte) in rho_slice.chunks_mut(8).enumerate() {
            byte.store_be(self.1[i]);
        }
        
        bitvec
    }
}

impl<const k: usize> MlKemDeserialize for MlkemEncapsulationKey<{k}> {
    fn deserialize(bitvec: &BitVec<u8, BitOrder>) -> Self {
        let (t_slice, rho_slice) = bitvec.split_at(256 * 12 * k);

        let mut t = Vector::new(RingRepresentation::Degree255);
        for (i, ring) in t_slice.chunks(256 * 12).enumerate() {
            t.data[i] = ByteDecode::<12>(ring);
        }

        let mut rho = [0u8; 32];
        for (i, chunk) in rho_slice.chunks(8).enumerate() {
            rho[i] = chunk.load_be();
        }

        (t, rho)
    }
}

impl<const k: usize> MlKemSerialize for MlkemDecapsulationKey<{k}> {
    fn serialize(&self) -> BitVec<u8, BitOrder> {
        let mut bitvec = bitvec![u8, BitOrder; 0; 768 * k + 96];

        // Serialize dk_pke
        for (i, chunk ) in bitvec[..384 * k].chunks_mut(384).enumerate() {
            ByteEncode::<12>(&self.0.data[i], chunk);
        }

        // Use our ek serialize implementation to get the serialized ek
        let serialized_ek = self.1.serialize();
        bitvec[384 * k..768 * k + 32].copy_from_bitslice(&serialized_ek);

        // Serialize hash
        bitvec[768 * k + 32..].copy_from_bitslice(&self.2.view_bits());

        // Serialize implicit rejection randomness
        bitvec[768 * k + 64..].copy_from_bitslice(&self.3.view_bits());


        bitvec
    }
}

impl<const k: usize> MlKemDeserialize for MlkemDecapsulationKey<{k}> {
    fn deserialize(bitvec: &BitVec<u8, BitOrder>) -> Self {
        let (dk_pke_slice, rest) = bitvec.split_at(384 * k);
        let (ek_slice, rest) = rest.split_at(384 * k + 32);
        let (hash_slice, z_slice) = rest.split_at(32);

        let mut dk_pke = Vector::new(RingRepresentation::NTT);
        for (i, chunk) in dk_pke_slice.chunks(12).enumerate() {
            dk_pke.data[i] = ByteDecode::<12>(chunk);
        }

        let ek = MlkemEncapsulationKey::<{k}>::deserialize(&BitVec::from_bitslice(ek_slice));

        let mut hash = [0u8; 32];
        for (i, byte) in hash_slice.chunks(8).enumerate() {
            hash[i] = byte.load_be();
        }

        let mut z = [0u8; 32];
        for (i, byte) in z_slice.chunks(8).enumerate() {
            z[i] = byte.load_be();
        }

        (dk_pke, ek, hash, z)        
    }
}

impl<const k: usize, const d_u: usize, const d_v: usize> MlKemSerialize for Cyphertext<{k}, {d_u}, {d_v}> where
    [(); 32*(d_u * k + d_v)]:
{
    fn serialize(&self) -> BitVec<u8, BitOrder> {
        let mut bitvec = bitvec![u8, BitOrder; 0; 32*(d_u * k + d_v)];

        let (c1_slice, c2_slice) = bitvec.split_at_mut(d_u * 32 * k);
        
        for (i, chunk) in c1_slice.chunks_mut(256 * d_u).enumerate() {
            ByteEncode::<{d_u}>(&self.0.0.data[i], chunk);
        }

        ByteEncode::<{d_v}>(&self.1.0, c2_slice);

        bitvec
    }
}

impl MlKemSerialize for Compressed<1, Ring> {
    fn serialize(&self) -> BitVec<u8, BitOrder> {
        let mut bitvec = bitvec![u8, BitOrder; 0; 256];
        ByteEncode::<1>(&self.0, bitvec.chunks_mut(256).next().unwrap());
        bitvec
    }
}

impl MlKemDeserialize for Compressed<1, Ring> {
    fn deserialize(bitvec: &BitVec<u8, BitOrder>) -> Self {
        let ring = ByteDecode::<1>(&bitvec);
        Compressed(ring)
    }
}