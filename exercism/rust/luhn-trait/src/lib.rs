pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?

impl<T> Luhn for T
where
    T: ToString,
{
    fn valid_luhn(&self) -> bool {
        let num = self.to_string();

        if num.chars().any(|c| !c.is_whitespace() && !c.is_digit(10)) {
            return false;
        }

        let digits: Vec<u32> = num
            .chars()
            .rev()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();

        if digits.len() <= 1 {
            return false;
        }

        fn luhn_digit(i: usize, d: &u32) -> u32 {
            return if i % 2 == 0 {
                *d
            } else if *d < 5 {
                d * 2
            } else {
                d * 2 - 9
            };
        }
        let checksum = digits
            .iter()
            .enumerate()
            .map(|(i, &d)| luhn_digit(i, &d))
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>();

        return checksum % 10 == 0;
    }
}
