use colored::Colorize;

pub enum LexingError {
    UnknownCharacter { line: usize, column: usize },
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexingError::UnknownCharacter { line, column } => {
                write!(
                    f,
                    "{} at {}, {}",
                    "unknown character".bold().red(),
                    format!("line {}", line).bold().red(),
                    format!("column {}", column).bold().red()
                )
            }
        }
    }
}

pub fn err_msg(error: LexingError) -> String {
    let prefix = "Failed in the lexing process,";

    let err_msg = match error {
        LexingError::UnknownCharacter { .. } => {
            format!("{error}")
        }
    };

    format!("{} {}", prefix, err_msg)
}
