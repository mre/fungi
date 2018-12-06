struct Magnitude {
    base: u64,
    text: String,
    size: u64,
}

static MAGNITUDE: [(u64, &str, u64); 7] = [
    (1_000_000_000_000_000_000, "quintillion", 1000),
    (1_000_000_000_000_000, "quadrillion", 1000),
    (1_000_000_000_000, "trillion", 1000),
    (1_000_000_000, "billion", 1000),
    (1_000_000, "million", 1000),
    (1000, "thousand", 1000),
    (100, "hundred", 10),
];

const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

// const TENS: [&str; 10] = [
//     "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
// ];

fn easy(num: u64) -> &'static str {
    if num >= 20 {
        panic!("");
    };
    return match num {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        n @ (14...19) => ONES[(n) as usize],
        _ => "",
    };
}

fn magnitude(num: u64) -> Option<Magnitude> {
    for m in MAGNITUDE.iter() {
        if num >= m.0 {
            return Some(Magnitude {
                base: m.0,
                text: m.1.to_string(),
                size: m.2,
            });
        }
    }
    None
}

fn tens(num: u64) -> &'static str {
    match num {
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => panic!(""),
    }
}

pub fn encode(num: u64) -> String {
    String::from(match num {
        n @ 0...19 => easy(n),
        20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 => tens(num),
        _ => match magnitude(num) {
            None => {
                if num / 10 > 0 {
                    let tens = ((num / 10) % 10) * 10;
                    return encode(tens) + "-" + encode(num - tens).as_str();
                }
                panic!("can't count to {}", num);
            }
            Some(m) => {
                let magnitude = (num / m.base) % m.size;
                let out = encode(magnitude) + " " + m.text.as_str();
                return match num % m.base {
                    0 => out,
                    n @ _ => out + " " + encode(n).as_str(),
                };
            }
        },
    })
}
