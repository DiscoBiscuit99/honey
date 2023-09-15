use crate::syntax::tokens::Token;

pub enum ParsingError<'t> {
    ExpectedFound {
        expected: Token,
        found: Option<Token>,
        position: usize,
    },
    ExpectedMsgFound {
        expected: &'t str,
        found: Option<Token>,
        position: usize,
    },
}

impl<'t> std::fmt::Display for ParsingError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::ExpectedFound {
                expected,
                found,
                position,
            } => {
                if let Some(found) = found {
                    write!(
                        f,
                        "expected {} at position {}, found {}",
                        expected, position, found
                    )
                } else {
                    write!(
                        f,
                        "expected {} at position {}, found nothing",
                        expected, position
                    )
                }
            }
            ParsingError::ExpectedMsgFound {
                expected,
                found,
                position,
            } => {
                if let Some(found) = found {
                    write!(
                        f,
                        "expected {} at position {}, found {}",
                        expected, position, found
                    )
                } else {
                    write!(
                        f,
                        "expected {} at position {}, found nothing",
                        expected, position
                    )
                }
            }
        }
    }
}
