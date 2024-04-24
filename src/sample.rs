use crate::ring::*;
use crate::params;
use crate::crypt;

use bitvec::prelude::*;



pub fn sample_ntt(mut xof_stream: crypt::XOF) -> Ring {
    
    let mut ring: Ring = Ring::ZEROES_NTT;
    let mut three_bytes = [0u8; 3];
    
    let mut j = 0;
    while j < 256 {
        //Each iteration samples 3 unfiormly random bytes total
        xof_stream.get_3_bytes(&mut three_bytes);

        let b1 = three_bytes[0] as u16;
        let b2 = three_bytes[1] as u16;
        let b3 = three_bytes[2] as u16;

        let d1 = b1 | (b2 & 0xF) << 8; // Uniform random sample of 12 bits
        let d2 = (b2 >> 4) | (b3 << 4); // Uniform random sample of 12 bits

        if d1 < params::Q {
            ring.data[j] = d1;
            j += 1;
        }

        if d2 < params::Q && j < 256 {
            ring.data[j] = d2;
            j += 1;
        }
    }

    ring
}



// Optimizaiton Attempt
// pub fn sample_poly_cbd_2(u32_random_array: [u32; 32]) -> Ring {
//     const QI: i16 = params::Q as i16;

//     let mut f: Ring = Ring::ZEROES_DEGREE255;
//     let mut i = 0;
//     for u32_random in u32_random_array.iter() {
//         let odd_digits = u32_random & 0x55555555;
//         let even_digits = u32_random & 0xAAAAAAAA;

//         let sum_1 = odd_digits + (even_digits >> 1);
        
//         let x = sum_1 & 0x33333333;
//         let y = ((((sum_1 >> 2) & 0x33333333) ^ 0x77777777) + 0x11111111) & 0x77777777;

//         let mut sum_2 = x + y;

//         for j in 0..8 {
//             let c0 = (sum_2 & 0x07) as u8;
//             let negative = (c0 & 0x04) == 0x04;

//             let c1 = match negative {
//                 false => c0 as i16, //positive
//                 true => -(((c0 - 1) ^ 0x07) as i16), //negative
//             };

//             f.data[i + j] = (QI + c1) as u16 % params::Q;

//             sum_2 >>= 4;
//         }

//         i += 8;
//     }

//     f
// }

pub fn sample_poly_cbd<const ETA: usize>(byte_array: [u8; 64*ETA]) -> Ring 
    where [u8; 64*ETA]:
{
    let b = byte_array.view_bits::<Lsb0>();
    let mut f: Ring = Ring::ZEROES_DEGREE255;

    for i in 0..256 {
        let mut x = 0u16;
        let mut y = 0u16;

        for j in 0..ETA {
            x += b[i*2*ETA + j] as u16;
            y += b[i*2*ETA + j + ETA] as u16;
        }

        f.data[i] = (x + params::Q - y) % params::Q;
    }

    f
}

#[cfg(test)]
mod test {

    pub fn test_byte_formula(byte: u8) -> (i8, i8) {

        println!("-----------------");
        println!("byte : {:08b}", byte);
    
        let odd_digits = byte & 0x55;
        let even_digits = byte & 0xAA;
    
        let sum_1 = odd_digits + (even_digits >> 1);
        println!("sum_1: {:08b}", sum_1);
            
        let x = sum_1 & 0x33;
        let y = ((((sum_1 >> 2) & 0x33) ^ 0x77) + 0x11) & 0x77;
        println!("x    : {:08b}", x);
        // println!(">>2  : {:08b}", sum_1 >> 2);
        // println!("&0x33: {:08b}", (sum_1 >> 2) & 0x33);
        // println!("^0x77: {:08b}", ((sum_1 >> 2) & 0x33) ^ 0x77);
        // println!("+0x11: {:08b}", ((((sum_1 >> 2) & 0x33) ^ 0x77) + 0x11));
        println!("y    : {:08b}", y);
    
        let sum_2 = x + y;
        println!("sum_2: {:08b}", sum_2);
    
        let a1 = (sum_2 >> 4) & 0x03;
        let a2 = sum_2        & 0x03;
    
        println!("a1   : {:04b}", a1);
        println!("a2   :     {:04b}", a2);
    
        let c1 = match (sum_2 & 0x40) == 0x40 {
            false => a1 as i8, //positive
            true => -(((a1 - 1) ^ 0x03) as i8), //negative
        };
    
        let c2 = match (sum_2 & 0x04) == 0x04 {
            false => a2 as i8, //positive
            true => -(((a2 - 1) ^ 0x03) as i8), //negative
        };
    
        println!("c1   : {:04b}", c1);
        println!("c2   :     {:04b}", c2);
    
        (c1, c2)
    }

    #[test]
    fn test_byte_coinflips_function(){
        assert_eq!(test_byte_formula(0b00000000), (0, 0));
        assert_eq!(test_byte_formula(0b00000001), (0, 1));
        assert_eq!(test_byte_formula(0b00000010), (0, 1));
        assert_eq!(test_byte_formula(0b00000011), (0, 2));
        assert_eq!(test_byte_formula(0b00000100), (0, -1));
        assert_eq!(test_byte_formula(0b00000101), (0, 0));
        assert_eq!(test_byte_formula(0b00000110), (0, 0));
        assert_eq!(test_byte_formula(0b00000111), (0, 1));
        assert_eq!(test_byte_formula(0b00001000), (0, -1));
        assert_eq!(test_byte_formula(0b00001001), (0, 0));
        assert_eq!(test_byte_formula(0b00001010), (0, 0));
        assert_eq!(test_byte_formula(0b00001011), (0, 1));
        assert_eq!(test_byte_formula(0b00001100), (0, -2));
        assert_eq!(test_byte_formula(0b00001101), (0, -1));
        assert_eq!(test_byte_formula(0b00001110), (0, -1));
        assert_eq!(test_byte_formula(0b00001111), (0, 0));
    }
}