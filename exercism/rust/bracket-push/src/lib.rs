pub struct Brackets(&'static str);

// static IN: [char; 3] = ['(', '[', '{'];
// static OUT: [char; 3] = [')', ']', '}'];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut balanced = true;
    let mut stack = Vec::with_capacity(string.len());
    string.chars().for_each(|x| match x {
        '}' => {
            if stack.pop() != Some('{') {
                balanced = false
            }
        }
        ']' => {
            if stack.pop() != Some('[') {
                balanced = false
            }
        }
        ')' => {
            if stack.pop() != Some('(') {
                balanced = false
            }
        }
        '{' | '[' | '(' => stack.push(x),
        _ => (),
    });
    if stack.len() > 0 {
        return false;
    };
    return balanced;
}
