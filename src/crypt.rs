use sha3::{Sha3_512, Sha3_256, Digest, Shake256, Shake128, digest::{ExtendableOutput, Update, XofReader}, Shake128Reader};
use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;

pub fn random_bytes<const n: usize> () -> [u8; n] {
    let mut rng = StdRng::from_entropy();
    let mut res = [0u8; n];
    rng.fill_bytes(&mut res);
    res
}

pub fn G<const L: usize>(c: [u8; L]) -> ([u8; 32], [u8; 32]) {
    let mut hasher = Sha3_512::new();
    Digest::update(&mut hasher, &c);
    let output = hasher.finalize();

    let (a,b) = output.split_at(32);
    (a.try_into().unwrap(), b.try_into().unwrap())
}

pub fn H(s: Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    Digest::update(&mut hasher, &s);
    let output = hasher.finalize();

    output.try_into().unwrap()
}

pub fn J(s: Vec<u8>) -> [u8; 32] {
    let mut hasher = Shake256::default();
    hasher.update(&s);
    let mut reader = hasher.finalize_xof();

    let mut res = [0u8; 32];
    reader.read(&mut res);
    res
}

pub fn prf<const eta: usize>(s: [u8; 32], b: u8) -> [u8; 64 * eta] 
    where [u8; 64 * eta]: 
{
    let mut hasher = Shake256::default();
    hasher.update(&s);
    hasher.update(&[b]);
    
    let mut reader = hasher.finalize_xof();
    let mut res = [0u8; 64 * eta];
    reader.read(&mut res);
    res
}


pub struct XOF {
    reader: Shake128Reader
}

impl XOF {
    pub fn new(p: [u8; 32], i: u8, j: u8) -> XOF {
        let mut hasher = Shake128::default();
        hasher.update(&p);
        hasher.update(&[i]);
        hasher.update(&[j]);
        XOF {
            reader: hasher.finalize_xof()
        }
    }

    pub fn get_3_bytes(&mut self) -> [u8; 3] {
        let mut res = [0u8; 3];
        self.reader.read(&mut res);
        res
    }
}