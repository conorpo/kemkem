
// Handles Compression and Byte Serialization
use crate::ring::*;
use crate::mlkem::*;
use bitvec::prelude::*;
use bitvec::access::*;

pub type BitOrder = Lsb0;

pub fn byte_encode<const D: usize>(f: &Ring, bitvec_slice: &mut BitSlice<BitSafeU8, BitOrder>) {
    assert_eq!(bitvec_slice.len(), 256 * D);

    for (slot, ele) in bitvec_slice.chunks_mut(D).zip(f.data.iter()) {
        slot.store_le(*ele)
    }
}

pub fn byte_decode<const D: usize>(bitvec_slice: &BitSlice<u8, BitOrder>, t: RingRepresentation) -> Ring {
    let mut f = match t {
        RingRepresentation::NTT => Ring::ZEROES_NTT,
        RingRepresentation::Degree255 => Ring::ZEROES_DEGREE255
    };

    assert_eq!(bitvec_slice.len(), 256 * D);

    for (slot, ele) in bitvec_slice.chunks(D).zip(f.data.iter_mut()) {
        *ele = slot.load_le();
    }
    f
}
pub trait MlKemSerialize {
    fn serialize(&self) -> BitVec<u8, BitOrder>;
}

pub trait MlKemDeserialize {
    fn deserialize(bitvec: &BitVec<u8, BitOrder>) -> Self;
}


impl<const K: usize> MlKemSerialize for MlkemEncapsulationKey<{K}> {
    fn serialize(&self) -> BitVec<u8, BitOrder> {
        let mut bitvec = bitvec![u8, BitOrder; 0; 8 * (384 * K + 32)];

        let (t_slice, rho_slice) = bitvec.split_at_mut(384 * K * 8);

        for (i, chunk) in t_slice.chunks_mut(384 * 8).enumerate() {
            byte_encode::<12>(&self.0.data[i], chunk);
        }

        // Serialize rho into last 32 bytes
        for (i, byte) in rho_slice.chunks_mut(8).enumerate() {
            byte.store_le(self.1[i])
        }
        
        bitvec
    }
}

impl<const K: usize> MlKemDeserialize for MlkemEncapsulationKey<{K}> {
    fn deserialize(bitvec: &BitVec<u8, BitOrder>) -> Self {
        let (t_slice, rho_slice) = bitvec.split_at(256 * 12 * K);

        let mut t: Vector<K> = Vector::new_ntt();
        for (i, ring) in t_slice.chunks(256 * 12).enumerate() {
            t.data[i] = byte_decode::<12>(ring, RingRepresentation::NTT);
        }

        let mut rho = [0u8; 32];
        for (i, chunk) in rho_slice.chunks(8).enumerate() {
            rho[i] = chunk.load_le();
        }

        (t, rho)
    }
}

impl<const K: usize> MlKemSerialize for MlkemDecapsulationKey<{K}> {
    fn serialize(&self) -> BitVec<u8, BitOrder> {
        let mut bitvec = bitvec![u8, BitOrder; 0; 8*(768 * K + 96)];
        let (dk_pke_slice, rest) = bitvec.split_at_mut(8*(384 * K));
        let (ek_slice, rest) = rest.split_at_mut(8*(384 * K + 32));
        let (hash_slice, z_slice) = rest.split_at_mut(8*32);


        // Serialize dk_pke
        for (i, chunk ) in dk_pke_slice.chunks_mut(8 * 384).enumerate() {
            byte_encode::<12>(&self.0.data[i], chunk);
        }

        // Use our ek serialize implementation to get the serialized ek
        let mut serialized_ek  = self.1.serialize();
        ek_slice.copy_from_bitslice(serialized_ek.split_at_mut(0).1);

        // Serialize hash
        for (i, byte) in hash_slice.chunks_mut(8).enumerate() {
            byte.store_le(self.2[i])
        }

        // Serialize implicit rejection randomness
        for (i, byte) in z_slice.chunks_mut(8).enumerate() {
            byte.store_le(self.3[i])
        }


        bitvec
    }
}

impl<const K: usize> MlKemDeserialize for MlkemDecapsulationKey<{K}> {
    fn deserialize(bitvec: &BitVec<u8, BitOrder>) -> Self {
        let (dk_pke_slice, rest) = bitvec.split_at(8*(384 * K));
        let (ek_slice, rest) = rest.split_at(8*(384 * K + 32));
        let (hash_slice, z_slice) = rest.split_at(8*(32));

        let mut dk_pke = Vector::new_ntt();
        for (i, chunk) in dk_pke_slice.chunks(8*384).enumerate() {
            dk_pke.data[i] = byte_decode::<12>(chunk, RingRepresentation::NTT);
        }

        let ek = MlkemEncapsulationKey::<{K}>::deserialize(&BitVec::from_bitslice(ek_slice));

        let mut hash = [0u8; 32];
        for (i, byte) in hash_slice.chunks(8).enumerate() {
            hash[i] = byte.load_le();
        }

        let mut z = [0u8; 32];
        for (i, byte) in z_slice.chunks(8).enumerate() {
            z[i] = byte.load_le();
        }

        (dk_pke, ek, hash, z)        
    }
}

impl<const K: usize, const D_U: usize, const D_V: usize> MlKemSerialize for MlKemCyphertext<{K}, {D_U}, {D_V}> where
    [(); 32*(D_U * K + D_V)]:
{
    fn serialize(&self) -> BitVec<u8, BitOrder> {
        let mut bitvec = bitvec![u8, BitOrder; 0; 256*(D_U * K + D_V)];

        let (c1_slice, c2_slice) = bitvec.split_at_mut(256 * D_U * K);

        for (i, chunk) in c1_slice.chunks_mut(256 * D_U).enumerate() {
            byte_encode::<{D_U}>(&self.0.0.data[i], chunk);
        }

        byte_encode::<{D_V}>(&self.1.0, c2_slice);

        bitvec
    }
}

impl<const K: usize, const D_U: usize, const D_V: usize> MlKemDeserialize for MlKemCyphertext<{K}, {D_U}, {D_V}> {
    fn deserialize(bitvec: &BitVec<u8, BitOrder>) -> Self {
        assert_eq!(bitvec.len(), 256 * (D_U * K + D_V));

        let (c1_slice, c2_slice) = bitvec.split_at(256 * D_U * K);

        let mut c1 = Vector::new_degree255();
        for (i, chunk) in c1_slice.chunks(256 * D_U).enumerate() {
            c1.data[i] = byte_decode::<D_U>(chunk, RingRepresentation::Degree255);
        }

        let c2 = byte_decode::<D_V>(c2_slice, RingRepresentation::Degree255);

        (Compressed::<D_U, Vector<K>>(c1), Compressed::<D_V, Ring>(c2))
    }
}

impl MlKemSerialize for Compressed<1, Ring> {
    fn serialize(&self) -> BitVec<u8, BitOrder> {
        let mut bitvec = bitvec![u8, BitOrder; 0; 256];
        byte_encode::<1>(&self.0, bitvec.chunks_mut(256).next().unwrap());
        bitvec
    }
}

impl MlKemDeserialize for Compressed<1, Ring> {
    fn deserialize(bitvec: &BitVec<u8, BitOrder>) -> Self {
        let ring = byte_decode::<1>(&bitvec, RingRepresentation::Degree255);
        Compressed(ring)
    }
}