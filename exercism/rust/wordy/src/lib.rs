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
    match components.len() {
        1 => return Some(components[0].parse::<i32>().expect("boom")),
        3 => {
            println!("components: {:?}", components);
            let a: i32 = components[0].parse::<i32>().expect("boom (a)");
            let b: i32 = components[2].parse::<i32>().expect("boom (b)");
            let operator: &str = components[1];
            match operator {
                "plus" => return Some(a + b),
                "minus" => return Some(a - b),
                "multiplied" => return Some(a * b),
                "divided" => return Some(a / b),
                _ => return None,
            }
        }
        4 => {
            println!("components: {:?}", components);
            let a: i32 = components[0].parse::<i32>().expect("boom (a)");
            let b: i32 = components[3].parse::<i32>().expect("boom (b)");
            let operator: &str = components[1];
            match operator {
                "multiplied" => return Some(a * b),
                "divided" => return Some(a / b),
                _ => return None,
            }
        }
        _ => return None,
    }
}
