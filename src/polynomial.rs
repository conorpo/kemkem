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

    pub fn mult(a_ring: &Ring, b_ring: &Ring) -> Ring {
        if let (
            Ring { data: a, t: RingRepresentation::NTT }, 
            Ring { data: b, t: RingRepresentation::NTT }
        ) = (a_ring, b_ring) {
                let mut result = Ring::new(RingRepresentation::NTT);

                for i in 0..128 {
                    let gamma = params::Zeta.pow(2); //replace with the bitrev thing

                    result.data[2*i] = (a[2*i] * b[2*i] + gamma * a[2*i + 1] * b[2*i + 1]) % 3329;
                    result.data[2*i + 1] = (a[2*i] * b[2*i + 1] + a[2*i + 1] * b[2*i]) % 3329;
                }

                return result;
        } else {
            panic!("Both rings must be in NTT form");
        }
    }

    pub fn ntt(&mut self) -> Ring {
        if let Ring { data, t: RingRepresentation::Degree255 } = self{
            let mut ntt_data = [0; 256];
            for i in 0..256 {
                for j in 0..256 {
                    ntt_data[i] = (ntt_data[i] + data[j] * W[i * j % 256]) % 3329;
                }
            }

            return Ring {
                data: ntt_data,
                t: RingRepresentation::NTT
            }
        } else {
            panic!("Ring is not in Degree255 form");
        }
    }

    pub fn inverse_ntt(&mut self) -> Ring {
        if let Polynomial::NTT(data) = self {
            let mut ring_data = [0; 256];
            for i in 0..256 {
                for j in 0..256 {
                    ring_data[i] = (ring_data[i] + data[j] * W[(256 - i) * j % 256]) % 3329;
                }
            }

            Polynomial {
                data: ring_data,
                t: PolynomialType::Ring
            }
        } else {
            panic!("Polynomial is not in NTT form");
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

    pub fn add(&mut self, other: &Vector<k>) -> Vector<k> {
        let mut result = Vector::new(self.data[0].t);
        for i in 0..k {
            result.data[i].add(&other.data[i]);
        }
        result
    }

    pub fn inner_product(&self, other: &Vector<k>) -> Ring {
        let mut result = Ring::new(self.data[0].t);
        for i in 0..k {
            result.add(&self.data[i].mul(&other.data[i]));
        }
        result
    }

    pub fn NTT(&self) -> Vector<k> {
        let mut result: Vector<k> = Vector::new(PolynomialType::NTT);
        for i in 0..k {
            result.data[i] = self.data[i].ntt();
        }
        result
    }
}

pub struct Matrix<const k: usize> {
    pub data: [[Polynomial; k]; k]
}

impl<const k: usize> Matrix<k> {
    pub fn new(t: PolynomialType) -> Matrix<k> {
        Matrix {
            data: [[Polynomial::new(t); k]; k]
        }
    }

    pub fn vector_multiply(&self, vector: &Vector<k>) -> Vector<k> {
        let mut result = Vector::new(self.data[0][0].t);
        for i in 0..k {
            for j in 0..k {
                result.data[i].add(&self.data[i][j].mul(&vector.data[j]));
            }
        }
        result
    }
}