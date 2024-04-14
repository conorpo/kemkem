use crate::{params, util::{self, bitrev7}};

#[derive(Clone, Copy, PartialEq)]
pub enum RingRepresentation {
    Degree255,
    NTT
}

#[derive(Clone)]
pub struct Ring {
    pub data: [u16; 256],
    pub t: RingRepresentation
}


impl Ring {
    pub fn new(t: RingRepresentation) -> Ring {
        Ring {
            data: [0; 256],
            t: t
        }
    }

    pub fn scalar_add(&mut self, value: u16) {
        for i in 0..256 {
            self.data[i] = (self.data[i] + value) % params::Q;
        }
    }
    
    pub fn scalar_mul(&mut self, value: u16) {
        for i in 0..256 {
            self.data[i] = (self.data[i] * value) % params::Q;
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
            let gamma = params::ZETA.pow(2*(bitrev7(i as u8) as u32) + 1);

            a[2*i] = (a[2*i] * b[2*i] + gamma * a[2*i + 1] * b[2*i + 1]) % params::Q;
            a[2*i + 1] = (a[2*i] * b[2*i + 1] + a[2*i + 1] * b[2*i]) % params::Q;
        }

        self
    }

    // In-Place, transforms ring to NTT form
    pub fn ntt(&mut self) -> &mut Self {
        if let Ring { data, t: RingRepresentation::Degree255 } = self {
            let mut k = 1;
            let mut len = 128;

            while len >= 2 {
                let mut start = 0;
                while start < 256 {
                    let zeta = params::ZETA.pow(util::bitrev7(k) as u32) % params::Q;

                    k += 1;

                    for j in start..(start + len) {
                        let t = (zeta * data[j + len]) % params::Q;
                        data[j + len] = (data[j] + params::Q - t) % params::Q;
                        data[j] = (data[j] + t) % params::Q;
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
        if let Ring {  data, t: RingRepresentation::NTT } = self {
            let mut k = 127;
            let mut len = 2;
            while len <= 128 {
                let mut start = 0;
                while start < 256 {
                    let zeta = params::ZETA.pow(util::bitrev7(k) as u32) % params::Q;

                    k -= 1;

                    for j in start..(start + len) {
                        let t = data[j];
                        data[j] = (data[j+len] + t) % params::Q;
                        data[j+len] = (zeta*(params::Q + t - data[j+len])) % params::Q;
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
#[derive(Clone)]
pub struct Vector<const k: usize> {
    pub data: [Ring; k]
}

impl<const k: usize> Vector<k> { 
    pub fn new(t: RingRepresentation) -> Vector<k> {
        Vector {
            data: core::array::from_fn(|_| Ring::new(t))
        }
    }

    pub fn add(&mut self, other: &Vector<k>) -> &mut Self {
        for i in 0..k {
            self.data[i].add(&other.data[i]);
        }
        self
    }

    pub fn inner_product(mut self, other: Vector<k>) -> Ring {
        let mut result = Ring::new(self.data[0].t);
        for i in 0..k {
            result.add(
                self.data[i].mult( &other.data[i] )
            );
        }
        result
    }   

    // NTT and NTT^-1 are in place and chainable
    pub fn ntt(mut self) -> Self {
        for i in 0..k {
            self.data[i].ntt();
        }
        self
    }

    pub fn inverse_ntt(mut self) -> Self {
        for i in 0..k {
            self.data[i].inverse_ntt();
        }
        self
    }
}

impl<const k: usize> PartialEq for Vector<k> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..k {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }
}

// Represent a compressed ring or vector as its own type
pub struct Compressed<const d: usize, T> 
{
    ring: T
}

impl<const d: usize> Compressed<d, Ring> {
    pub fn compress(mut ring: Ring) -> Compressed<d, Ring> {
        for i in 0..256 {
            ring.data[i] = ((ring.data[i] as u32) * 2u32.pow(d as u32) / params::Q as u32) as u16;
        }
        
        Compressed {
            ring: ring
        }
    }

    pub fn decompress(self) -> Ring {
        let Compressed { mut ring } = self;

        for i in 0..256 {
            ring.data[i] = ((ring.data[i] as u32) * params::Q as u32 / 2u32.pow(d as u32)) as u16;
        }
        
        ring
    }
}

impl<const d: usize, const k: usize> Compressed<d, Vector<k>> {
    pub fn compress(mut vector: Vector<k>) -> Compressed<d, Vector<k>> {
        for i in 0..k {
            for j in 0..256 {
                vector.data[i].data[j] = ((vector.data[i].data[j] as u32) * 2u32.pow(d as u32) / params::Q as u32) as u16;
            }
        }
        
        Compressed {
            ring: vector
        }
    }

    pub fn decompress(self) -> Vector<k> {
        let Compressed { mut ring } = self;

        for i in 0..k {
            for j in 0..256 {
                ring.data[i].data[j] = ((ring.data[i].data[j] as u32) * params::Q as u32 / 2u32.pow(d as u32)) as u16;
            }
        }
        
        ring
    }
}


pub struct Matrix<const k: usize> {
    pub data: [[Ring; k]; k]
}

impl<const k: usize> Matrix<k> {
    pub fn new(t: RingRepresentation) -> Matrix<k> {
        Matrix {
            data: core::array::from_fn(|_| core::array::from_fn(|_| Ring::new(t)))
        }
    }

    // These matrix operations are only ever done once, so they can be consuming on the matrix
    pub fn right_vector_multiply(mut self, vector: &Vector<k>) -> Vector<k> {
        let mut result = Vector::new(self.data[0][0].t);
        for i in 0..k {
            for j in 0..k {
                result.data[i].add(self.data[i][j].mult(&vector.data[j]));  
            }
        }
        result
    }

    pub fn left_vector_multiply(mut self, vector: &Vector<k>) -> Vector<k> {
        let mut result = Vector::new(self.data[0][0].t);
        for i in 0..k {
            for j in 0..k {
                result.data[i].add(self.data[j][i].mult(&vector.data[j]));
            }
        }
        result
    }
}