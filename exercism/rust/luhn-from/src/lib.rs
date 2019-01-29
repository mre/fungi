pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if self.code.len() <= 1 {
            return false;
        }

        let mut cs: u32 = 0;
        let mut ix: u32 = 0;
        for c in self.code.chars().rev() {
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
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(val: T) -> Luhn {
        Luhn{
            code: val.to_string()
            // code: val.to_string()
            //     .chars()
            //     .filter(|c| !c.is_whitespace())
            //     .map(|c| c.to_digit(10))
            //     .collect(),
        }
    }
}
