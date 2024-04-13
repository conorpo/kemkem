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

// Should these be in place?
pub fn Compress<const d: usize>(F: &Ring) -> Ring {
    let mut compressed = Ring::new(F.t);
    for (new, old) in compressed.data.iter_mut().zip(F.data.iter()) {
        *new = ((*old as u32) * 2u32.pow(d as u32) / Q as u32) as u16;
    }
    compressed
}

pub fn Decompress<const d: usize>(F: &Ring) -> Ring {
    let mut decompressed = Ring::new(F.t);
    for (new, old) in decompressed.data.iter_mut().zip(F.data.iter()) {
        *new = ((*old as u32) * Q as u32 / 2u32.pow(d as u32)) as u16;
    }
    decompressed
}


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
pub trait MlKemSerialize<const k: usize> {
    fn serialize(&self) -> BitVec<u8, BitOrder>;
}

impl<const k: usize> MlKemSerialize<k> for KPKE_EncryptionKey<k> {
    fn serialize(&self) -> BitVec<u8, BitOrder> {
        let mut bitvec = bitvec![u8, BitOrder; 0; k * 256 * 12 + 32];
        for (i, chunk) in bitvec.chunks_mut(256 * 12).enumerate() {
            ByteEncode::<12>(&self.0.data[i], chunk);
        }

        // Serialize rho into last 32 bytes
        bitvec[256 * 12 * k..].chunks_mut(8).enumerate().for_each(|(i, chunk)| {
            chunk.store_be(self.1[i]);
        });

        bitvec
    }
}