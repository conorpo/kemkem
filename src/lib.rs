#![allow(incomplete_features)]

#![feature(generic_const_exprs)]
#![feature(portable_simd)]

mod crypt;
#[macro_use]
mod util;

pub mod params;

mod ring;
mod sample;
pub mod serialize;

mod kpke;
pub use kpke::Cyphertext;

#[cfg(test)]
mod seeded_test;


pub mod mlkem;