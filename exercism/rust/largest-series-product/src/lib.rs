#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    } else if span == 0 {
        return Ok(1);
    }

    let mut v: Vec<u32> = Vec::with_capacity(string_digits.len());
    for c in string_digits.chars() {
        if !c.is_numeric() {
            return Err(Error::InvalidDigit(c));
        } else {
            v.push(c.to_digit(10).unwrap());
        }
    }

    // noice onliners... stolen online.
    // sequence.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>().windows(n).map(|window| window.iter().product()).max().unwrap())
    // input.chars().map(|c| c.to_digit(10)).collect::<Option<Vec<_>>>().and_then(|v| v.windows(length).map(|w| w.iter().product()).max()).ok_or(())
    //
    // https://doc.rust-lang.org/std/primitive.slice.html#method.windows
    // https://doc.rust-lang.org/std/iter/trait.Product.html#tymethod.product

    let mut max: u64 = u64::min_value();
    let mut i: usize = 0;
    let mut j: usize = span;
    let mut p: u64;
    let mut slc: &[u32] = &v[i..j];
    loop {
        println!("slice: {:?} [{}..{}]", slc, i, j);
        p = slc.iter().fold(1u64, |prod, val| prod * (*val as u64));
        max = if p > max { p } else { max };
        i += 1;
        j += 1;
        if j > v.len() {
            break;
        }
        slc = &v[i..j];
    }
    return Ok(max);
}
