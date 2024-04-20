# `kemkem`

A rust implementation of **ML-KEM**, **M**odular **L**attice-based **K**ey **E**ncapsulation **M**echanism. This is a post-quantum assymetric encryption scheme for sharing keys, and its difficulty is based on the hardness of the Modular Learning With Errors (M-LWE) problem.

It features:
- An intuitive API, where parameters are deobfuscated and serialization is handle explicitly
- Support for all 3 parameter sets 
  - MlKem512
  - MlKem768
  - MlKem1024
- An implementation generic for all 3 possible parameter sets (Const Generic Expressions)
- Benchmarking, and simulation bins, both with and without serialization

## !! **IMPORTANT** !!
This project has been made with security in mind, but has not be independently audited. The tests garuntee that each of the main ML-KEM functions (KeyGen, Encaps, Decaps) work as expected, and the simulate test runs the entire process with a pre-defined seed, and compares intermediate outputs against a publicly available set. 

However, it was shown that FIPS 203, the standard that this implementation is based on, **is currently vulnerable to some timing attacks on the key generation and implicit rejection randomness generation.** And as of now, my implementation is subject to the same drawbacks.

So all that being said, please use at your own risk, with a strong reccomendation to not use in production.

## Usage
The scheme consists of 3 main steps, key generation from party A, shared key encapsulation from party B, and decapsulation of that shared key from party A.
```rust
use kemkem::{mlkem::*, params::*};
use kemkem::serialize::*; // For byte encoding

// Key Generation (Party A)
let (ek, dk) = key_gen::<MlKem1024>();

// Encapsulation (Party B)
let (key, c) = encaps::<MlKem1024>(ek);

// Decapsulation (Party A)
let key = decaps::<MlKem1024>(c,dk);
```

Serialization is needed to actually send the data between parties, kemkem provides traits for this.
```rust
// Key Generation (Party A)
let (ek, dk) = key_gen::<MlKem1024>();

let ek_bytes = ek.serialize();

// Encapsulation (Party B)
let ek = MlkemEncapsulationKey::<{MlKem1024::K}>::deserialize(&ek_bytes);

let (key, c) = encaps::<MlKem1024>(ek);

let c_bytes = c.serialize();

// Decapsulation (Party A)
let c = MlKemCyphertext::<{MlKem1024::K}, {MlKem1024::D_U}, {MlKem1024::D_V}>::deserialize(&c);

let key = decaps::<MlKem1024>(c,dk);

```

Parameter choice can be made dynamic with a simple match
```rust
let (ek, dk) = match param_set {
    MlKemParamSet::MlKem512 => key_gen::<MlKem512>(),
    MlKemParamSet::MlKem768 => key_gen::<MlKem768>(),
    MlKemParamSet::MlKem1024 => key_gen::<MlKem1024>(),
};
```

## Benchmarks


| Parameter Set | Simulation Time (µs) |
|---------------|-------------|
| `MlKem512`      | `288.10`   |   
|`MlKem768`| `468.24`|
|`MlKem1024`| `700.63` |
##### *Without Serialization