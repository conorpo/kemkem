use crate::params;

#[derive(Clone, Copy)]
pub enum RingRepresentation {
    Degree255,
    NTT
}

pub struct Ring {
    pub data: [u16; 256],
    t: RingRepresentation
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
            self.data[i] = (self.data[i] + value) % 3329;
        }
    }
    
    pub fn scalar_mul(&mut self, value: u16) {
        for i in 0..256 {
            self.data[i] = (self.data[i] * value) % 3329;
        }
    }

    pub fn add(&mut self, other: &Ring) {
        for i in 0..256 {
            self.data[i] = (self.data[i] + other.data[i]) % 3329;
        }
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

                result.data[2*i] = (a[2*i] * b[2*i] + gamma * a[2*i + 1] * b[2*i + 1]) % 3329;
                result.data[2*i + 1] = (a[2*i] * b[2*i + 1] + a[2*i + 1] * b[2*i]) % 3329;
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
                    todo!();
                    let zeta = params::Zeta.pow(k); 

                    k += 1;

                    for j in start..(start + len) {
                        let t = (zeta * data[j + len]) % 3329;
                        data[j + len] = (data[j] + 3329 - t) % 3329;
                        data[j] = (data[j] + t) % 3329;
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

    pub fn inverse_ntt(&mut self) -> &mut Self {
        if let Ring { data, t: RingRepresentation::NTT } = self {
            let mut k = 127;
            let mut len = 2;
            while len <= 128 {
                let mut start = 0;
                while start < 256 {
                    todo!();
                    let zeta = params::Zeta.pow(k); 

                    k -= 1;

                    for j in start..(start + len) {
                        let t = data[j];
                        data[j] = (data[j+len] + t) % 3329;
                        data[j+len] = (zeta*(3329 + t - data[j+len])) % 3329;
                    }

                    start += 2 * len;
                }

                len *= 2;
            }

            self.scalar_mul(3303);

            self.t = RingRepresentation::Degree255;

            return self;
        } else {
            panic!("Ring is already in Degree255 form");
        }
    }
}

//Tuple that is either all ring or all ntt, k is length
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