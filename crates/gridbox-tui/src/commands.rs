#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserCommand {
    Help,
    Quit,
    Clear,
    Live,
    Refresh,
    Model(String),
    Driver(u32),
    Schedule(u16),
    Session {
        year: u16,
        event: String,
        session: String,
    },
    Compare {
        year: u16,
        event: String,
        session: String,
        drivers: Vec<String>,
    },
    Ask(String),
}

pub fn parse_command(input: &str) -> Result<UserCommand, String> {
    let input = input.trim();
    if input.is_empty() {
        return Err("empty command".to_string());
    }
    if !input.starts_with('/') {
        return Ok(UserCommand::Ask(input.to_string()));
    }

    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts[0] {
        "/help" => Ok(UserCommand::Help),
        "/quit" | "/exit" => Ok(UserCommand::Quit),
        "/clear" => Ok(UserCommand::Clear),
        "/live" => Ok(UserCommand::Live),
        "/refresh" => Ok(UserCommand::Refresh),
        "/model" if parts.len() >= 2 => Ok(UserCommand::Model(parts[1..].join(" "))),
        "/driver" if parts.len() == 2 => parts[1]
            .parse()
            .map(UserCommand::Driver)
            .map_err(|_| "usage: /driver <number>".to_string()),
        "/schedule" if parts.len() == 2 => parts[1]
            .parse()
            .map(UserCommand::Schedule)
            .map_err(|_| "usage: /schedule <year>".to_string()),
        "/session" if parts.len() == 4 => Ok(UserCommand::Session {
            year: parts[1]
                .parse()
                .map_err(|_| "year must be numeric".to_string())?,
            event: parts[2].to_string(),
            session: parts[3].to_string(),
        }),
        "/compare" if parts.len() >= 6 => Ok(UserCommand::Compare {
            year: parts[1]
                .parse()
                .map_err(|_| "year must be numeric".to_string())?,
            event: parts[2].to_string(),
            session: parts[3].to_string(),
            drivers: parts[4..].iter().map(|value| value.to_string()).collect(),
        }),
        "/session" => Err("usage: /session <year> <event> <session>".to_string()),
        "/compare" => {
            Err("usage: /compare <year> <event> <session> <driver1> <driver2>".to_string())
        }
        unknown => Err(format!("unknown command: {unknown}")),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_command, UserCommand};

    #[test]
    fn plain_text_becomes_ai_question() {
        assert_eq!(
            parse_command("Who is in the pit window?").unwrap(),
            UserCommand::Ask("Who is in the pit window?".to_string())
        );
    }

    #[test]
    fn parses_compare() {
        let command = parse_command("/compare 2025 Monaco Q NOR VER").unwrap();
        assert!(matches!(command, UserCommand::Compare { .. }));
    }
}
