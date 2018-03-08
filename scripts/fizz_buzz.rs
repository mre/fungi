use std::borrow::Cow;

pub fn fizzbuzz(n: u32) {
    for i in 1..n + 1 {
        println!("{}", fizzbuzz_string(i));
    }
}

fn fizzbuzz_string(i: u32) -> Cow<'static, str> {
    let by3 = i % 3 == 0;
    let by5 = i % 5 == 0;
    if by3 && by5 {
        "FizzBuzz".into()
    } else if by3 {
        "Fizz".into()
    } else if by5 {
        "Buzz".into()
    } else {
        format!("{}", i).into()
    }
}

#[cfg(test)]
mod tests {
    use super::fizzbuzz_string;

    #[test]
    fn single_numbers() {
        assert_eq!("1", fizzbuzz_string(1));
        assert_eq!("2", fizzbuzz_string(2));
        assert_eq!("Fizz", fizzbuzz_string(3));
        assert_eq!("Buzz", fizzbuzz_string(5));
        assert_eq!("7", fizzbuzz_string(7));
        assert_eq!("Fizz", fizzbuzz_string(9));
        assert_eq!("Buzz", fizzbuzz_string(10));
        assert_eq!("FizzBuzz", fizzbuzz_string(15));
        assert_eq!("Fizz", fizzbuzz_string(18));
        assert_eq!("Buzz", fizzbuzz_string(20));
        assert_eq!("23", fizzbuzz_string(23));
        assert_eq!("FizzBuzz", fizzbuzz_string(30));
        assert_eq!("32", fizzbuzz_string(32));
        assert_eq!("Buzz", fizzbuzz_string(50));
        assert_eq!("FizzBuzz", fizzbuzz_string(90));
        assert_eq!("Buzz", fizzbuzz_string(100));
    }
}
