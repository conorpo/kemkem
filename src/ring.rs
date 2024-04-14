use crate::{params, util};

#[derive(Clone, Copy)]
pub enum RingRepresentation {
    Degree255,
    NTT
}

#[derive(Clone, Copy)]
pub struct Ring {
    pub data: [u16; 256],
    pub t: RingRepresentation
}

// Most Ring operations are inplace

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

    pub fn mult(ring_a: &Ring, ring_b: &Ring) -> Ring {
        if let (
            Ring { data: a, t: RingRepresentation::NTT }, 
            Ring { data: b, t: RingRepresentation::NTT }
        ) = (ring_a, ring_b) {
            let mut result = Ring::new(RingRepresentation::NTT);

            for i in 0..128 {
                todo!();
                let gamma = params::Zeta.pow(2); //replace with the bitrev thing

                result.data[2*i] = (a[2*i] * b[2*i] + gamma * a[2*i + 1] * b[2*i + 1]) % params::Q;
                result.data[2*i + 1] = (a[2*i] * b[2*i + 1] + a[2*i + 1] * b[2*i]) % params::Q;
            }
            result
        } else {
            panic!("Both rings must be in NTT form");
        }
    }

    // In-Place, transforms ring to NTT form
    pub fn ntt(&mut self) -> &mut Self{
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
            
            return self;
        } else {
            panic!("Ring is already in NTT form");
        }
    }

    pub fn inverse_ntt(&mut self) -> Self {
        if let Ring { data, t: RingRepresentation::NTT } = self {
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

            return *self;
        } else {
            panic!("Ring is already in Degree255 form");
        }
    }

    pub fn compress(&mut self, d: usize) -> &mut Self {
        for i in 0..256 {
            self.data[i] = ((self.data[i] as u32) * 2u32.pow(d as u32) / params::Q) as u16;
        }
        self
    }

    pub fn decompress(&mut self, d: usize) -> &mut Self {
        for i in 0..256 {
            self.data[i] = ((self.data[i] as u32) * params::Q / 2u32.pow(d as u32)) as u16;
        }
        self
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
#[derive(Clone, Copy)]
pub struct Vector<const k: usize> {
    pub data: [Ring; k]
}

impl<const k: usize> Vector<k> { 
    pub fn new(t: RingRepresentation) -> Vector<k> {
        Vector {
            data: core::array::from_fn(|_| Ring::new(t))
        }
    }

    pub fn add(&self, other: &Vector<k>) -> Vector<k> {
        let mut result = Vector::new(self.data[0].t);
        for i in 0..k {
            result.data[i].add(&other.data[i]);
        }
        result
    }

    pub fn inner_product(&self, other: &Vector<k>) -> Ring {
        let mut result = Ring::new(self.data[0].t);
        for i in 0..k {
            result.add(&Ring::mult(&self.data[i], &other.data[i]));
        }
        result
    }   


    // NTT and NTT^-1 are in place and chainable
    pub fn ntt(&mut self) -> &mut Self {
        for i in 0..k {
            self.data[i].ntt();
        }
        self
    }

    pub fn inverse_ntt(&mut self) -> &mut Self {
        for i in 0..k {
            self.data[i].inverse_ntt();
        }
        self
    }

    pub fn compress(&mut self, d: usize) -> &mut Self {
        for i in 0..k {
            self.data[i].compress(d);
        }
        self
    }

    pub fn decompress(&mut self, d: usize) -> &mut Self {
        for i in 0..k {
            self.data[i].decompress(d);
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

pub struct Matrix<const k: usize> {
    pub data: [[Ring; k]; k]
}

impl<const k: usize> Matrix<k> {
    pub fn new(t: RingRepresentation) -> Matrix<k> {
        Matrix {
            data: core::array::from_fn(|_| core::array::from_fn(|_| Ring::new(t)))
        }
    }

    pub fn right_vector_multiply(&self, vector: &Vector<k>) -> Vector<k> {
        let mut result = Vector::new(self.data[0][0].t);
        for i in 0..k {
            for j in 0..k {
                result.data[i].add(&Ring::mult(&self.data[i][j], &vector.data[j]));  
            }
        }
        result
    }

    pub fn left_vector_multiply(&self, vector: &Vector<k>) -> Vector<k> {
        let mut result = Vector::new(self.data[0][0].t);
        for i in 0..k {
            for j in 0..k {
                result.data[i].add(&Ring::mult(&self.data[j][i], &vector.data[j]));
            }
        }
        result
    }
}