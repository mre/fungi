use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

const NUMERALS: [(u32, &str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut n = self.0;
        for (v, s) in NUMERALS.iter().cloned() {
            while n >= v {
                write!(f, "{}", s)?;
                n -= v;
            }
        }
        Ok(())
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(num)
    }
}
