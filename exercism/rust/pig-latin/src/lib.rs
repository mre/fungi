// - Rule 1: If a word begins with a vowel sound, add an "ay" sound
//   to the end of the word. Please note that "xr" and "yt" at the
//   beginning of a word make vowel sounds (e.g. "xray" -> "xrayay",
//   "yttria" -> "yttriaay").
// - Rule 2: If a word begins with a consonant sound, move it to the
//   end of the word and then add an "ay" sound to the end of the
//   word. Consonant sounds can be made up of multiple consonants,
//   a.k.a. a consonant cluster (e.g. "chair" -> "airchay").
// - Rule 3: If a word starts with a consonant sound followed by
//   "qu", move it to the end of the word, and then add an "ay" sound to
//   the end of the word (e.g. "square" -> "aresquay").
// - Rule 4: If a word contains a "y" after a consonant cluster or
//   as the second letter in a two letter word it makes a vowel sound
//   (e.g. "rhythm" -> "ythmrhay", "my" -> "ymay").

fn to_pig(input: &str) -> String {
    // Returns the byte index of the first character of this string
    // slice that matches the pattern.
    let l: usize = input.len();
    if l == 2 && &input[l - 1..] == "y" {
        // - Rule 4: If a word contains a "y" as the second letter
        //   in a two letter word it makes a vowel sound (e.g. "my"
        //   -> "ymay").
        return format!("y{}ay", &input[1..2]);
    }
    if &input[0..2] == "xr" || &input[0..2] == "yt" {
        //   Rule 1: Please note that "xr" and "yt" at the
        //   beginning of a word make vowel sounds (e.g. "xray" -> "xrayay",
        //   "yttria" -> "yttriaay").
        return format!("{}ay", &input);
    }
    let v: Option<usize> = input.find(|c: char| "aeiou".contains(c));
    match v {
        None => {
            if let Some(i) = input.find(|c: char| "y".contains(c)) {
                // - Rule 4: If a word contains a "y" after a consonant cluster
                //   it makes a vowel sound  (e.g. "rhythm" -> "ythmrhay")
                return format!("{}{}ay", &input[i..], &input[..i]);
            } else {
                return input.to_owned();
            }
        }
        Some(mut i) => {
            if i == 0 {
                // - Rule 1: If a word begins with a vowel sound, add an "ay" sound
                //   to the end of the word.
                return format!("{}ay", &input);
            }
            if i >= 1 && &input[i - 1..i + 1] == "qu" {
                // - Rule 3: If a word starts with a consonant sound followed by
                //   "qu", move it to the end of the word, and then add an
                //   "ay" sound to the end of the word (e.g. "square" ->
                //   "aresquay").
                //
                i += 1;
            }
            // - Rule 2: If a word begins with a consonant sound, move
            //   it to the end of the word and then add an "ay" sound to
            //   the end of the word. Consonant sounds can be made up of
            //   multiple consonants, a.k.a. a consonant cluster
            //   (e.g. "chair" -> "airchay").
            return format!("{}{}ay", &input[i..], &input[..i]);
        }
    };
}

pub fn translate(input: &str) -> String {
    input
        .to_lowercase()
        .split_whitespace()
        .map(to_pig)
        .collect::<Vec<String>>()
        .join(" ")
}
