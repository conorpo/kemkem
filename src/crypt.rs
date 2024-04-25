use sha3::{digest::{core_api::XofReaderCoreWrapper, ExtendableOutput, Update, XofReader}, Digest, Sha3_256, Sha3_512, Shake128ReaderCore, Shake256};
use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;

pub fn random_bytes<const N: usize> () -> [u8; N] {
    let mut rng = StdRng::from_entropy();
    let mut res = [0u8; N];
    rng.fill_bytes(&mut res);
    res
}

pub fn g<const L: usize>(c: &[u8; L]) -> ([u8; 32], [u8; 32]) {
    let mut hasher = Sha3_512::new();
    Digest::update(&mut hasher, c);
    let output = hasher.finalize();

    let (a,b) = output.split_at(32);
    (a.try_into().unwrap(), b.try_into().unwrap())
}

pub fn h(s: &Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    Digest::update(&mut hasher, s);
    let output = hasher.finalize();

    output.try_into().unwrap()
}

pub fn j(s: Vec<u8>) -> [u8; 32] {
    let mut hasher = Shake256::default();
    hasher.update(&s);
    let mut reader = hasher.finalize_xof();

    let mut res = [0u8; 32];
    XofReader::read(&mut reader, &mut res);
    res
}

pub fn prf<const ETA: usize>(s: &[u8; 32], b: u8) -> [u8; 64 * ETA] 
{
    let mut hasher = Shake256::default();
    hasher.update(s);
    hasher.update(&[b]);
    
    let mut reader = hasher.finalize_xof();
    
    let mut res = [0u8; 64 * ETA];
    XofReader::read(&mut reader, &mut res);
    res
}

// Optimization Attempt
// pub fn prf_2(s: &[u8; 32], b: u8) -> [u32; 32]
// {
//     let mut hasher = Shake256::default();
//     hasher.update(s);
//     hasher.update(&[b]);
    
//     let mut reader = hasher.finalize_xof();
    
//     let mut res = U32U8Union { u32: [0u32; 32] };
//     unsafe { 
//         XofReader::read(&mut reader, &mut res.u8);
//         res.u32
//     }
// }



pub struct XOF {
    reader: XofReaderCoreWrapper<Shake128ReaderCore>
}

impl XOF {
    pub fn new(p: &[u8], i: u8, j: u8) -> XOF {
        let mut hasher = sha3::Shake128::default();
        hasher.update(p);
        hasher.update(&[i, j]);
        XOF {
            reader: hasher.finalize_xof()
        }
    }

    pub fn get_3_bytes(&mut self, buf: &mut [u8; 3]) {
        XofReader::read(&mut self.reader, buf);
    }
}