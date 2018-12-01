pub fn verse(n: i32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} {} of beer on the wall.\n",n, n, n-1, if n == 2 { "bottle"} else { "bottles" }),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    (end..(start + 1))
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<String>>()
        .join("\n")
}
