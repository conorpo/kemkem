use sha3::{Sha3_512, Sha3_256, Digest, Shake256, Shake128, digest::{ExtendableOutput, Update, XofReader}, Shake128Reader};
use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;

pub fn random_bytes(n: usize) -> Vec<u8> {
    let mut rng = StdRng::from_entropy();
    let mut res = vec![0u8; n];
    rng.fill_bytes(&mut res);
    res
}

pub fn G(c: Vec<u8>) -> ([u8; 32], [u8; 32]) {
    let mut hasher = Sha3_512::new();
    hasher.update(&c);
    let mut output = [0u8; 64];
    hasher.finalize();

    let (a, b) = output.split_at(32).unwrap();
}

pub fn H(s: Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(&s);
    let output = hasher.finalize();
    output
}

pub fn J(s: Vec<u8>) -> [u8; 32] {
    let mut hasher = Shake256::default();
    hasher.update(&s);

    let mut reader = hasher.finalize_xof();
    let mut res = [0u8; 32];
    reader.read(&mut res).unwrap();
    res
}

pub fn prf_2(s: [u8; 32], b: u8) -> [u8; 128] {
    let mut hasher = Shake256::default();
    hasher.update(s.to_be_bytes());
    hasher.update(b);
    
    let mut reader = hasher.finalize_xof();
    let mut res = [0u8; 128];
    reader.read(&mut res).unwrap();
    res
}

pub fn prf_3(s: [u8; 32], b: u8) -> [u8; 192] {
    let mut hasher = Shake128::default();
    hasher.update(s.to_be_bytes());
    hasher.update(b);
    
    let mut reader = hasher.finalize_xof();
    let mut res = [0u8; 192];
    reader.read(&mut res).unwrap();
    res
}

pub struct XOF {
    reader: Shake128Reader
}

impl XOF {
    pub fn new(p: [u8; 32], i: u8, j: u8) -> XOF {
        let mut hasher = Shake128::default();
        hasher.update(p.to_be_bytes());
        hasher.update(i);
        hasher.update(j);
        XOF {
            reader: hasher.finalize_xof()
        }
    }

    pub fn get_3_bytes(&mut self) -> [u8; 3] {
        let mut res = [0u8; 3];
        self.reader.read(&mut res).unwrap();
        res
    }
}