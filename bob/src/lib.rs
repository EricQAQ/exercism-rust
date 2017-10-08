pub fn reply(message: &str) -> &str {
    let msg = message.trim();
    match (msg.is_empty(), msg.ends_with("?"), msg.to_uppercase() == msg && msg.chars().any(|ch| ch.is_alphabetic())) {
        (true, _, _) => "Fine. Be that way!",
        (false, true, false) => "Sure.",
        (false, _, true) => "Whoa, chill out!",
        _ => "Whatever."
    }
}
