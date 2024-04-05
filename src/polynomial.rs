pub enum Polynomial {
    Ring([u16; 256]),
    NTT([u16; 256]),
}

impl Polynomial {
    pub fn new() -> Polynomial {
        Polynomial::Ring([0; 256])
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

    pub fn ntt(&mut self) -> Polynomial::NTT {
        if let Polynomial::Ring(data) = self {
            let mut ntt_data = [0; 256];
            for i in 0..256 {
                for j in 0..256 {
                    ntt_data[i] = (ntt_data[i] + data[j] * W[i * j % 256]) % 3329;
                }
            }
            Polynomial::NTT(ntt_data)
        } else {
            panic!("Polynomial is not in Ring form");
        }
    }

    pub fn inverse_ntt(&mut self) -> Polynomial::Ring {
        if let Polynomial::NTT(data) = self {
            let mut ring_data = [0; 256];
            for i in 0..256 {
                for j in 0..256 {
                    ring_data[i] = (ring_data[i] + data[j] * W[(256 - i) * j % 256]) % 3329;
                }
            }
            Polynomial::Ring(ring_data)
        } else {
            panic!("Polynomial is not in NTT form");
        }
    }
}

//Tuple that is either all ring or all ntt, k is length
pub struct Vector<Type: Polynomial,k: usize> {
    data: [Type; k]
}

impl<Type: Polynomial,k: usize> Vector<Type,k> {
    pub fn new() -> Vector<Type,k> {
        Vector {
            data: [Type::new(); k]
        }
    }

    pub fn add(&mut self, other: &Vector<Type,k>) {
        for i in 0..k {
            self.data[i].add(&other.data[i]);
        }
    }

    pub fn inner_product(&self, other: &Vector<Type,k>) -> Type {
        let mut result = Type::new();
        for i in 0..k {
            result.add(&self.data[i].mul(&other.data[i]));
        }
        result
    }
}

pub struct Matrix<T: Polynomial> {
    data: Vec<Vec<T>>
}

impl<Type: Polynomial Matrix<T> {
    pub fn new(k: usize) -> Matrix<T> {
        Matrix {
            data: vec![vec![T::new(); k]; k]
        }
    }

    pub fn vector_multiply(&self, vector: &Vector<Type,k>) -> Vector<Type,k> {
        let mut result = Vector::new();
        for i in 0..k {
            for j in 0..k {
                result.data[i].add(&self.data[i][j].mul(&vector.data[j]));
            }
        }
        result
    }
}

static ka: usize = 256;

pub struct TestMatrix {
    data: [[u16; ka]; ka]
};

impl TestMatrix {
    pub fn new() -> TestMatrix {
        TestMatrix {
            data: [[0; ka]; ka]
        }
    }

    pub fn vector_multiply(&self, vector: &TestVector) -> TestVector {
        let mut result = TestVector::new();
        for i in 0..ka {
            for j in 0..ka {
                result.data[i] = (result.data[i] + self.data[i][j] * vector.data[j]) % 3329;
            }
        }
        result
    }
}