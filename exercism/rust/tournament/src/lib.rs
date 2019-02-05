use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    // this is the lazy version: no struct, one-letter variables.
    // 
    // - MP: Matches Played
    // - W: Matches Won
    // - D: Matches Drawn (Tied)
    // - L: Matches Lost
    // - P: Points

    let mut t: HashMap<&str, (i32, i32, i32, i32, i32)> = HashMap::new();
    let mut n: usize = 0;
    for l in match_results.lines() {
        n += 1;
        let mut i = l.split(';');
        let a = i.next().unwrap();
        let b = i.next().unwrap();
        // MP
        t.entry(a).or_insert((0, 0, 0, 0, 0)).0 += 1;
        t.entry(b).or_insert((0, 0, 0, 0, 0)).0 += 1;

        match i.next().unwrap() {
            "win" => {
                // W
                t.entry(a).or_insert((0, 0, 0, 0, 0)).1 += 1;
                // L
                t.entry(b).or_insert((0, 0, 0, 0, 0)).3 += 1;
                // P
                t.entry(a).or_insert((0, 0, 0, 0, 0)).4 += 3;
            }
            "loss" => {
                // W
                t.entry(b).or_insert((0, 0, 0, 0, 0)).1 += 1;
                // L
                t.entry(a).or_insert((0, 0, 0, 0, 0)).3 += 1;
                // P
                t.entry(b).or_insert((0, 0, 0, 0, 0)).4 += 3;
            }
            "draw" => {
                // D
                t.entry(a).or_insert((0, 0, 0, 0, 0)).2 += 1;
                t.entry(b).or_insert((0, 0, 0, 0, 0)).2 += 1;
                // P
                t.entry(a).or_insert((0, 0, 0, 0, 0)).4 += 1;
                t.entry(b).or_insert((0, 0, 0, 0, 0)).4 += 1;
            }
            _ => panic!("Wrong format for file"),
        }
    }

    let mut o: Vec<String> = Vec::with_capacity(n + 1);
    let h: &str = "Team                           | MP |  W |  D |  L |  P";
    o.push(h.to_string());

    let mut tv: Vec<(&str, (i32, i32, i32, i32, i32))> = Vec::with_capacity(t.len());
    for (k, v) in t {
        tv.push((k, v));
    }
    tv.sort_by(|a, b| (b.1).4.cmp(&(a.1).4));
    for v in tv.iter() {
        o.push(format!(
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            v.0, (v.1).0, (v.1).1, (v.1).2, (v.1).3, (v.1).4
        ));
    }
    return o.join("\n");
}
