pub fn bitrev7(x: u8) -> u8 {
    let mut x = x;
    let mut y = 0;
    for _ in 0..7 {
        y = (y << 1) | (x & 1);
        x >>= 1;
    }
    y
}