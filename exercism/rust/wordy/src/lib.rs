pub struct WordProblem;

const QUESTION_START: &str = "What is ";
const QUESTION_MARK: &str = "?";

pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with(QUESTION_START) {
        return None;
    }

    if !command.ends_with(QUESTION_MARK) {
        return None;
    }

    let operation: &str = command
        .trim_start_matches(QUESTION_START)
        .trim_end_matches(QUESTION_MARK);

    let components: Vec<&str> = operation.split(' ').collect::<Vec<&str>>();
    let mut n: Vec<i32> = Vec::new();
    let mut o: Option<fn(i32, i32) -> i32> = None;

    for c in components {
        if c.parse::<i32>().is_ok() {
            n.push(c.parse::<i32>().expect("boom on number"));
            if o.is_some() {
                let b = n.pop();
                let a = n.pop();
                n.push(o.unwrap()(a.unwrap(), b.unwrap()));
                o = None;
            }
            if n.len() > 1 && o.is_none() {
                return None;
            }
        } else {
            match c {
                "plus" => {
                    // TODO: reject multiple operation for all
                    if o.is_some() || n.len() == 0 {
                        return None;
                    } else {
                        o = Some(|a, b| a + b)
                    }
                }
                "minus" => {
                    // TODO: reject multiple operation for all
                    if o.is_some() || n.len() == 0 {
                        return None;
                    } else {
                        o = Some(|a, b| a - b)
                    }
                }
                "multiplied" => o = Some(|a, b| a * b),
                "divided" => o = Some(|a, b| a / b),
                "by" => {
                    // TODO: check if the previous is multiplied or divided
                    continue;
                }
                _ => {
                    return None;
                }
            }
        }
    }
    if o.is_some() {
        return None;
    }
    return n.pop();
}
