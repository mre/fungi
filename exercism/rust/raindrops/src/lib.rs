pub fn boring(n: u32) -> String {
    let mut rainstring = String::new();

    if n % 3 == 0 {
        rainstring += "Pling";
    }
    if n % 5 == 0 {
        rainstring += "Plang";
    }
    if n % 7 == 0 {
        rainstring += "Plong";
    }

    if rainstring == "" {
        return n.to_string();
    } else {
        return rainstring;
    }
}

static WORDS: [(usize, &'static str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: usize) -> String {
    match WORDS
        .iter()
        .filter(|(d, _)| n % d == 0)
        .map(|&(_, s)| s)
        .collect::<String>()
    {
        ref res if !res.is_empty() => res.to_owned(),
        _ => n.to_string(),
    }
}
