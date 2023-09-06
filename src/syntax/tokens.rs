use colored::Colorize;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Mut,
    NumberKeyword,
    UnitKeyword,
    WhereKeyword, // 🤔
    Equal,
    Colon,
    SemiColon,
    Comma,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Arrow,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Identifier(String),
    NumberLiteral(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Let => write!(f, "keyword '{}'", "let".bold()),
            Token::Mut => write!(f, "keyword '{}'", "mut".bold()),
            Token::NumberKeyword => write!(f, "keyword '{}'", "number".bold()),
            Token::UnitKeyword => write!(f, "keyword '{}'", "unit".bold()),
            Token::WhereKeyword => write!(f, "where '{}'", "where".bold()),
            Token::Equal => write!(f, "equal '{}'", "=".bold()),
            Token::Colon => write!(f, "colon '{}'", ":".bold()),
            Token::SemiColon => write!(f, "semi colon '{}'", ";".bold()),
            Token::Comma => write!(f, "comma '{}'", ",".bold()),
            Token::Plus => write!(f, "plus '{}'", "+".bold()),
            Token::Minus => write!(f, "minus '{}'", "-".bold()),
            Token::Asterisk => write!(f, "asterisk '{}'", "*".bold()),
            Token::Slash => write!(f, "slash '{}'", "/".bold()),
            Token::Arrow => write!(f, "arrow '{}'", "->".bold()),
            Token::OpenBrace => write!(f, "open brace '{}'", "{".bold()),
            Token::CloseBrace => write!(f, "closing brace '{}'", "}".bold()),
            Token::OpenParen => write!(f, "open parenthesis '{}'", "(".bold()),
            Token::CloseParen => write!(f, "closing parenthesis '{}'", ")".bold()),
            Token::Identifier(ident) => write!(f, "identifier '{}'", format!("{}", ident).bold()),
            Token::NumberLiteral(literal) => {
                write!(f, "number '{}'", format!("{}", literal).bold())
            }
        }
    }
}
