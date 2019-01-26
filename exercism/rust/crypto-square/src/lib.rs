// The plaintext should be organized in to a rectangle.  The size of the
// rectangle (`r x c`) should be decided by the length of the message,
// such that `c >= r` and `c - r <= 1`, where `c` is the number of columns
// and `r` is the number of rows.
fn sides(l: usize) -> Option<(usize, usize)> {
    let c: usize = (l as f64).sqrt().ceil() as usize;
    let r = (l as f64).sqrt() as usize;
    assert!(c >= r);
    assert!(c - r <= 1);
    assert!(c * r >= l);
    return Some((c, r));
}

pub fn encrypt(input: &str) -> String {
    let inp = input
        .to_lowercase()
        .chars()
        .filter(|&c| c.is_alphanumeric())
        .collect::<String>();
    let (height, width): (usize, usize) = sides(inp.len()).expect("boom");

    println!("building a square height: {} and width: {}", height, width);
    let mut rct: Vec<String> = vec![String::with_capacity(width); height];

    let mut i_itr: std::str::Chars<'_> = inp.chars();
    for _ in 0..width {
        for h in 0..height {
            match i_itr.next() {
                Some(ch) => {
                    rct[h].push(ch)
                }
                None => rct[h].push(' '),
            }
        }
    }

    for h in 0..height {
        let r = rct[h].chars().collect::<Vec<char>>();
        for w in 0..width {
            if w >= r.len() {
                break;
            }
            print!("[{}]", r[w]);
        }
        println!("");
    }

    return rct.join(" ");
}
