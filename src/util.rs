use crate::params::{Q, ZETA};

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
            result = (result * base) % Q as u32;
        }
        exp = exp >> 1;
        base = (base * base) % Q as u32;
    }
    result as u16
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}


#[cfg(test)]
mod tests {
    use super::*;

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