// let number_of_digits = (x as f64).log10() as u32 + 1;
fn digits(mut num: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits
}

pub fn is_armstrong_number(num: u32) -> bool {
    let number_of_digits = digits(num).len();
    let mut sum = 0;
    let mut temp = num;
    while temp > 0 {
        sum += (temp % 10).pow(number_of_digits as u32);
        temp /= 10;
    }
    num == sum
}
