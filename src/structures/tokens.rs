use colored::Colorize;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Mut,
    If,
    Else,
    NumberKeyword,
    UnitKeyword,
    WhereKeyword, // 🤔
    Assignment,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Negate,
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
    OpenAngle,
    CloseAngle,
    Identifier(String),
    NumberLiteral(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Let => write!(f, "keyword '{}'", "let".bold()),
            Token::Mut => write!(f, "keyword '{}'", "mut".bold()),
            Token::If => write!(f, "keyword '{}'", "if".bold()),
            Token::Else => write!(f, "keyword '{}'", "else".bold()),
            Token::NumberKeyword => write!(f, "keyword '{}'", "number".bold()),
            Token::UnitKeyword => write!(f, "keyword '{}'", "unit".bold()),
            Token::WhereKeyword => write!(f, "where '{}'", "where".bold()),
            Token::Assignment => write!(f, "assignment '{}'", "=".bold()),
            Token::Negate => write!(f, "negation '{}'", "!".bold()),
            Token::Equal => write!(f, "equal '{}'", "==".bold()),
            Token::NotEqual => write!(f, "not equal '{}'", "==".bold()),
            Token::LessEqual => write!(f, "less-than-or-equal-to '{}'", "=".bold()),
            Token::GreaterEqual => write!(f, "greater-than-or-equal-to '{}'", "=".bold()),
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
            Token::OpenAngle => write!(f, "open angle bracket / less than '{}'", "<".bold()),
            Token::CloseAngle => write!(f, "closing angle bracket / greater than '{}'", ">".bold()),
            Token::Identifier(ident) => write!(f, "identifier '{}'", format!("{}", ident).bold()),
            Token::NumberLiteral(literal) => {
                write!(f, "number '{}'", format!("{}", literal).bold())
            }
        }
    }
}
