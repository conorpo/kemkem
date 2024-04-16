use crate::params;
use crate::util::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RingRepresentation {
    Degree255,
    NTT
}

#[derive(Clone, Debug)]
pub struct Ring {
    pub data: [u16; 256],
    pub t: RingRepresentation
}


impl ToString for Ring {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..256 {
            s.push_str(format!("{:X}", self.data[i]).as_str());
            s.push(',');
        }
        s
    }
}


impl Ring {
    pub fn new(t: RingRepresentation) -> Ring {
        Ring {
            data: [0; 256],
            t: t
        }
    }

    pub fn scalar_mul(&mut self, value: u16) {
        let val = value as u32;

        for i in 0..256 {
            self.data[i] = ((self.data[i] as u32 * val) % params::Q as u32) as u16;
        }
    }

    pub fn add(&mut self, other: &Ring) -> &mut Self {
        for i in 0..256 {
            self.data[i] = (self.data[i] + other.data[i]) % params::Q;
        }
        self
    }

    pub fn sub(&mut self, other: &Ring) -> &mut Self {
        for i in 0..256 {
            self.data[i] = (self.data[i] + params::Q - other.data[i]) % params::Q;
        }
        self
    }

    pub fn mult(&mut self, other: &Ring) -> &mut Self {
        if self.t != RingRepresentation::NTT || other.t != RingRepresentation::NTT {
            panic!("Multiplication requires NTT form");
        }

        let a = &mut self.data;
        let b = &other.data;

        for i in 0usize..128usize {
            let gamma = fastmodpow(params::ZETA, 2*bitrev7(i as u8) + 1) as u64;

            let r_1: u64 = (a[2*i] as u64) * (b[2*i] as u64) + gamma * (a[2*i + 1] as u64) * (b[2*i + 1] as u64); 
            let r_2: u64 = (a[2*i] as u64) * (b[2*i + 1] as u64) + (a[2*i + 1] as u64) * (b[2*i] as u64);

            a[2*i] = (r_1 % params::Q as u64) as u16;
            a[2*i + 1] = (r_2 % params::Q as u64) as u16;
        }

        self
    }

    // In-Place, transforms ring to NTT form
    pub fn ntt(&mut self) -> &mut Self {
        const Q32: u32 = params::Q as u32;

        if let Ring { data, t: RingRepresentation::Degree255 } = self {
            let mut k = 1;
            let mut len = 128;

            while len >= 2 {
                let mut start = 0;
                while start < 256 {
                    let zeta = fastmodpow(params::ZETA, bitrev7(k)) as u32;

                    k += 1;

                    for j in start..(start + len) {
                        let t = (zeta * data[j + len] as u32) % Q32;
                        data[j + len] = ((data[j] as u32 + Q32 - t) % Q32) as u16;
                        data[j] = ((data[j] as u32 + t) % Q32) as u16;
                    }

                    start += 2 * len;
                }

                len /= 2;
            }

            self.t = RingRepresentation::NTT;
            
            self
        } else {
            panic!("Ring is already in NTT form");
        }
    }

    pub fn inverse_ntt(&mut self) -> &mut Self {
        const Q32: u32 = params::Q as u32;

        if let Ring {  data, t: RingRepresentation::NTT } = self {
            let mut k = 127;
            let mut len = 2;
            while len <= 128 {
                let mut start = 0;
                while start < 256 {
                    let zeta = fastmodpow(params::ZETA, bitrev7(k)) as u32;

                    k -= 1;

                    for j in start..(start + len) {
                        let t = data[j];
                        data[j] = (data[j+len] + t) % params::Q;
                        data[j+len] = ((zeta * (params::Q + t - data[j+len]) as u32) % params::Q as u32) as u16;
                    }

                    start += 2 * len;
                }

                len *= 2;
            }

            self.scalar_mul(3303);

            self.t = RingRepresentation::Degree255;

            self
        } else {
            panic!("Ring is already in Degree255 form");
        }
    }
}

impl PartialEq for Ring {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..256 {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }
}

//Tuple that is either all ring or all ntt, k is length
#[derive(Clone, Debug)]
pub struct Vector<const K: usize> {
    pub data: [Ring; K]
}

impl<const K: usize> Vector<K> { 
    pub fn new(t: RingRepresentation) -> Vector<K> {
        Vector {
            data: core::array::from_fn(|_| Ring::new(t))
        }
    }

    pub fn add(&mut self, other: &Vector<K>) -> &mut Self {
        for i in 0..K {
            self.data[i].add(&other.data[i]);
        }
        self
    }

    pub fn inner_product(mut self, other: Vector<K>) -> Ring {
        let mut result = Ring::new(self.data[0].t);
        for i in 0..K {
            result.add(
                self.data[i].mult( &other.data[i] )
            );
        }
        result
    }   

    // NTT and NTT^-1 are in place and chainable
    pub fn ntt(mut self) -> Self {
        for i in 0..K {
            self.data[i].ntt();
        }
        self
    }

    pub fn inverse_ntt(mut self) -> Self {
        for i in 0..K {
            self.data[i].inverse_ntt();
        }
        self
    }
}

impl<const K: usize> PartialEq for Vector<K> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..K {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }
}

// Represent a compressed ring or vector as its own type

#[derive(PartialEq, Clone, Debug)]
pub struct Compressed<const D: usize, T> (pub T);

pub trait CompressionConstants<const D: usize> {
    const POW_HALF: u32;
    const BITMASK: u16;
}

impl<const D: usize, T> CompressionConstants<D> for Compressed<D, T> {
    const POW_HALF: u32 = 1 << (D - 1);
    const BITMASK: u16 = (1 << D) - 1;
}

const Q_HALF: u32 = params::Q32 / 2;

impl<const D: usize> Compressed<D, Ring> {
    pub fn compress(mut ring: Ring) -> Compressed<D, Ring> {
        for i in 0..256 {
            ring.data[i] = ((((ring.data[i] as u32) << D) + Q_HALF) / params::Q32) as u16 & Compressed::<D, Ring>::BITMASK;
        }
        
        Compressed (ring)
    }

    pub fn decompress(self) -> Ring {
        let Compressed (mut ring) = self;

        for i in 0..256 {
            ring.data[i] = (((ring.data[i] as u32) * params::Q32 + Compressed::<D, Ring>::POW_HALF) >> D) as u16;
        }
        
        ring
    }
}

impl<const D: usize, const K: usize> Compressed<D, Vector<K>> {
    pub fn compress(mut vector: Vector<K>) -> Compressed<D, Vector<K>> {
        for i in 0..K {
            for j in 0..256 {
                vector.data[i].data[j] = ((((vector.data[i].data[j] as u32) << D) + Q_HALF) / params::Q32) 
                    as u16 & Compressed::<D, Vector<K>>::BITMASK;
            }
        }
        
        Compressed (vector)
    }

    pub fn decompress(self) -> Vector<K> {
        let Compressed (mut vector) = self;

        for i in 0..K {
            for j in 0..256 {
                vector.data[i].data[j] = (((vector.data[i].data[j] as u32) * params::Q32 + Compressed::<D, Vector<K>>::POW_HALF) >> D) as u16;
            }
        }
        
        vector
    }
}


#[derive(Clone, Debug)]
pub struct Matrix<const K: usize> {
    pub data: [[Ring; K]; K]
}

impl<const K: usize> Matrix<K> {
    pub fn new(t: RingRepresentation) -> Matrix<K> {
        Matrix {
            data: core::array::from_fn(|_| core::array::from_fn(|_| Ring::new(t)))
        }
    }

    // These matrix operations are only ever done once, so they can be consuming on the matrix
    pub fn right_vector_multiply(mut self, vector: &Vector<K>) -> Vector<K> {
        let mut result = Vector::new(self.data[0][0].t);
        for i in 0..K {
            for j in 0..K {
                result.data[i].add(self.data[i][j].mult(&vector.data[j]));  
            }
        }
        result
    }

    pub fn left_vector_multiply(mut self, vector: &Vector<K>) -> Vector<K> {
        let mut result = Vector::new(self.data[0][0].t);
        for i in 0..K {
            for j in 0..K {
                result.data[i].add(self.data[j][i].mult(&vector.data[j]));
            }
        }
        result
    }
}