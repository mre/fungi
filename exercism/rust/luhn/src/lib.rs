/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() <= 1 {
        return false;
    }

    let mut cs: u32 = 0;
    let mut ix: u32 = 0;
    for c in code.chars().rev() {
        if c.is_whitespace() {
            continue;
        }
        if c.is_numeric() {
            ix += 1;
            let mut v: u32 = c.to_digit(10).expect("this should not happen");
            if ix % 2 == 0 {
                v = v * 2;
            }
            v = if v > 9 { v - 9 } else { v };
            cs = cs + v;
        } else {
            return false;
        }
    }

    if ix <= 1 {
        return false;
    }
    
    if cs % 10 == 0 {
        return true;
    }
    return false;
}
