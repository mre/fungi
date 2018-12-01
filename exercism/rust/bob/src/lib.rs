fn am_yelling(message: &str) -> bool {
    message.to_uppercase() == message && message.chars().any(|c| c.is_uppercase())
}

fn am_asking(message: &str) -> bool {
    message.ends_with("?")
}

fn said_nothing(message: &str) -> bool {
    message.is_empty()
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if am_yelling(m) & am_asking(m) => "Calm down, I know what I'm doing!",
        m if am_asking(m) => "Sure.",
        m if am_yelling(m) => "Whoa, chill out!",
        m if said_nothing(m) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
