use crate::params::Q32;

pub fn bitrev7(x: u8) -> u8 {
    let mut x = x;
    let mut y = 0;
    for _ in 0..7 {
        y = (y << 1) | (x & 1);
        x >>= 1;
    }
    y
}

pub fn fastmodpow(base: u16, exp: u8) -> u16 {
    let mut base = base as u32; // As mod is around 2^12, this garuntees no overflow
    let mut exp = exp as u32; 
    let mut result = 1;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % Q32;
        }
        exp = exp >> 1;
        base = (base * base) % Q32;
    }
    result as u16
}

// pub const MONT_R: i32 = 1 << 16; // 65536
// //pub const MONT_MULT = 20

// pub const BARRET_R : i64 = 1 << 26; // 67108864
// pub const BARRET_MULT : i64 = 20159;



// fn barret_reduce(x: u16) -> u16 {
//     let t =
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::ZETA;

    #[test]
    fn test_bitrev7() {
        assert_eq!(127, bitrev7(127));
        assert_eq!(1, bitrev7(64));
        assert_eq!(64, bitrev7(1));
        assert_eq!(0, bitrev7(0));
    }

    #[test]
    fn test_fastmodpow() {
        assert_eq!(fastmodpow(ZETA, 1), ZETA);
        assert_eq!(fastmodpow(ZETA, 2), ZETA * ZETA);
        assert_eq!(fastmodpow(ZETA, 10), 650);
        assert_eq!(fastmodpow(ZETA, 100), 1476);
        assert_eq!(fastmodpow(ZETA, 255), 1175);
    }
}