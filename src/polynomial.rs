pub enum PolynomialType {
    Ring,
    NTT
}

pub struct Polynomial {
    data: [u16; 256],
    t: PolynomialType
}

impl Polynomial {
    pub fn new(t: PolynomialType) -> Polynomial {
        Polynomial {
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

    pub fn add(&mut self, other: &Polynomial) {
        for i in 0..256 {
            self.data[i] = (self.data[i] + other.data[i]) % 3329;
        }
    }

    pub fn ntt(&mut self) -> Polynomial {
        if let Polynomial::Ring(data) = self {
            let mut ntt_data = [0; 256];
            for i in 0..256 {
                for j in 0..256 {
                    ntt_data[i] = (ntt_data[i] + data[j] * W[i * j % 256]) % 3329;
                }
            }

            Polynomial {
                data: ntt_data,
                t: PolynomialType::NTT
            }
        } else {
            panic!("Polynomial is not in Ring form");
        }
    }

    pub fn inverse_ntt(&mut self) -> Polynomial {
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
    data: [Polynomial; k]
}

impl<const k: usize> Vector<k> { 
    pub fn new(t: PolynomialType) -> Vector<k> {
        Vector {
            data: [Polynomial::new(t); k]
        }
    }

    pub fn add(&mut self, other: &Vector<k>) -> Vector<k> {
        let mut result = Vector::new(self.data[0].t);
        for i in 0..k {
            result.data[i].add(&other.data[i]);
        }
        result
    }

    pub fn inner_product(&self, other: &Vector<k>) -> Polynomial {
        let mut result = Polynomial::new(self.data[0].t);
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
    data: [[Polynomial; k]; k]
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