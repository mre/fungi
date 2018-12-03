pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }

    let mut output: Vec<String> = vec![];
    for i in 0..(list.len() - 1) {
        output.push(format!(
            "For want of a {} the {} was lost.",
            list[i],
            list[i + 1]
        ));
    }

    if list.len() >= 3 && list[0..3] == ["nail", "shoe", "horse"] {
        output.push(String::from("And all for the want of a nail."));
    } else {
        output.push(format!("And all for the want of a {}.", list[0]));
    }
    output.join("\n")
}
