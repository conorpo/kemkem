# `kemkem`

- [crates.io](https://crates.io/crates/kemkem)
- [docs.rs](https://docs.rs/kemkem)


A rust implementation of **ML-KEM**, **M**odular **L**attice-based **K**ey **E**ncapsulation **M**echanism. This is a post-quantum assymetric encryption scheme for sharing keys, and its difficulty is based on the hardness of the Modular Learning With Errors (M-LWE) problem.

It features:
- An intuitive API, where parameters are deobfuscated and serialization is handled explicitly
- Support for all 3 parameter sets 
  - MlKem512
  - MlKem768
  - MlKem1024
- An implementation generic for all 3 possible parameter sets (Const Generic Expressions)
- Binaries to simulate the entire process with and without serialization, aswell as benchmarks to track performance of indidivual steps.

## !! **IMPORTANT** !!
This project has been made with security in mind, but has not be independently audited. The tests garuntee that each of the main ML-KEM functions (KeyGen, Encaps, Decaps) work as expected, and the `seeded_test.rs` tests runs the entire process with a pre-defined seed, and compares intermediate outputs against a publicly available set. 

However, it was shown that FIPS 203, the standard that this implementation is based on, **is currently vulnerable to some timing attacks on the key generation and implicit rejection randomness generation.** And as of now, my implementation is subject to the same drawbacks.

So all that being said, please use at your own risk, with a strong reccomendation to not use in production.

## Usage
The scheme consists of 3 main steps, key generation from party A, shared key encapsulation from party B, and decapsulation of that shared key from party A.
```rust
use kemkem::{mlkem::*, params::*};

// Key Generation (Party A)
let (ek, dk) = key_gen::<MlKem1024>();

// Encapsulation (Party B)
let (key, c) = encaps::<MlKem1024>(ek);

// Decapsulation (Party A)
let key = decaps::<MlKem1024>(c,dk);
```

Serialization is needed to actually send the data between parties, kemkem provides traits for this.
```rust
use kemkem::{mlkem::*, params::*, serialize:: *};

// Key Generation (Party A)
let (ek, dk) = key_gen::<MlKem1024>();

let ek_bytes = ek.serialize();

// Encapsulation (Party B)
let ek = MlkemEncapsulationKey::<{MlKem1024::K}>::deserialize(&ek_bytes);

let (key, c) = encaps::<MlKem1024>(ek);

let c_bytes = c.serialize();

// Decapsulation (Party A)
let c = MlKemCyphertext::<{MlKem1024::K}, {MlKem1024::D_U}, {MlKem1024::D_V}>::deserialize(&c_bytes);

let key = decaps::<MlKem1024>(c,dk);

```

Parameter choice can be made dynamic with a simple match
```rust
let (ek, dk) = match param_set {
    "512" => key_gen::<MlKem512>(),
    "768" => key_gen::<MlKem768>(),
    "1024" => key_gen::<MlKem1024>(),
    _ => panic!("Invalid parameter set")
};
```

## Benchmarks
For the MlKem768 parameter set:
| Step | Time (µs) |
|---------------|-------------|
| `KeyGen`      | `78.3`   |   
|`Encaps`| `82.2`|
|`Decaps`| `104.7` |
*Without Serialization

This was run on my AMD Ryzen 7 1700X, and can be replicated with `cargo bench`. I've decided not to include other implementations in the table, as I am new to rust and can't ensure I've set up a fair comparison. But running `libcrux`'s ML-KEM-768 benchmark on my machine I got very similair times except for Decaps which ran in `~70µs`.