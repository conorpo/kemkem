use bitvec::prelude::*;
use crate::polynomial::*;

pub struct Encoded<const d: usize> where
    [u8; 32*d]: { // Garuntees that the array is not over the max size
    data: [u8; 32*d]
}


pub fn ByteEncode<const d: usize>(F: Polynomial) -> Encoded<d> where [u8; 32*d]: 
{
    let mut b: [u8; 32*d] = [0; 32*d];
    for (i, chunk) in b.chunks_mut(d).enumerate() {
        let mut a = F.data[i];
        chunk.
    }
    // Convert bitvec to byte array
    b.as_raw_slice().try_into().unwrap()
}