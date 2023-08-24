use crate::lexer::tokens::{Keyword, Special, Token};

mod types;
use types::*;

/// Parses the given tokens and outputs an abstract syntax tree.
pub fn parse(tokens: Vec<Token>) -> SyntaxTree {
    parse_program(tokens)
}

/// Takes a collection of tokens and produces an abstract syntax tree representing the whole
/// program.
fn parse_program(tokens: Vec<Token>) -> SyntaxTree {
    let (tree, _) = parse_statement_list(tokens);
    tree
}

fn parse_statement_list(tokens: Vec<Token>) -> (SyntaxTree, Vec<Token>) {
    let mut statements = vec![];

    let (statement_tree, tokens) = parse_statement(tokens);
    let statement = match statement_tree {
        SyntaxTree::Statement(statement) => statement,
        _ => panic!("Failed to parse statement list: not a statement"),
    };

    statements.push(statement);

    if !tokens.is_empty() {
        let (statements_tree, _) = parse_statement_list(tokens.clone());
        let mut more_statements = match statements_tree {
            SyntaxTree::StatementList { statements } => statements,
            _ => panic!("Failed to parse statement list, expected more statements"),
        };
        statements.append(&mut more_statements);
    }

    let tree = SyntaxTree::StatementList { statements };

    (tree, tokens)
}

fn parse_statement(tokens: Vec<Token>) -> (SyntaxTree, Vec<Token>) {
    parse_declaration(tokens)
}

/// Takes a collection of tokens and produces a declaration tree.
fn parse_declaration(tokens: Vec<Token>) -> (SyntaxTree, Vec<Token>) {
    let mut tokens = tokens.iter();

    // Parse the "let" keyword first.
    let let_keyword = tokens
        .next()
        .expect("Failed to parse declaration tree: empty token stream");

    let let_keyword = match let_keyword {
        Token::Keyword(Keyword::Let) => Keyword::Let,
        _ => panic!("Failed to parse declaration tree: no \"let\"?"),
    };

    // Parse the identifier.
    let identifier = tokens
        .next()
        .expect("Failed to parse declaration tree: no identifier");

    let identifier = match identifier {
        Token::Identifier(ident) => ident,
        _ => panic!("Failed to parse declaration tree: token not an identifier?"),
    };

    // Parse the type annotation.
    let tokens: Vec<Token> = tokens.map(|t| t.clone()).collect();
    let (type_annotation, tokens) = parse_type_annotation(tokens);
    let mut tokens = tokens.iter();

    // Parse the assignment.
    let assignment = tokens
        .next()
        .expect("Failed to parse declaration: no assignment token?");

    let _assignment = match assignment {
        Token::Special(Special::Assignment) => Special::Assignment,
        _ => panic!("Failed to parse declaration: expected assignment, found {assignment:?}"),
    };

    // Parse the expression to be assigned.
    let tokens = tokens.map(|t| t.clone()).collect();
    let (expression, tokens) = parse_expression(tokens);

    let expression = match expression {
        SyntaxTree::Expression(exp) => exp,
        _ => panic!("Failed to parse declaration: expected expression, found {expression:?}"),
    };

    let mut tokens = tokens.iter();

    // Parse the full stop (the statement stop, i.e. the ';').
    let _full_stop = tokens
        .next()
        .expect("Failed to parse declaration: no full stop?");

    let tree = SyntaxTree::Statement(Statement::Declaration {
        keyword: let_keyword,
        identifier: identifier.clone(),
        type_annotation,
        expression,
    });

    let tokens = tokens.map(|t| t.clone()).collect();

    (tree, tokens)
}

fn parse_type_annotation(tokens: Vec<Token>) -> (TypeAnnotation, Vec<Token>) {
    let mut tokens = tokens.iter();

    // Parse the special ":" character.
    let colon = tokens
        .next()
        .expect("Failed to parse type annotation: no colon prefix?");

    let colon = match colon {
        Token::Special(Special::Colon) => Special::Colon,
        _ => panic!("Failed to parse type annotation: wrong prefix {colon:?}"),
    };

    // Parse the type.
    let data_type = tokens
        .next()
        .expect("Failed to parse type annotation: no data type?");

    let data_type = match data_type {
        Token::DataType(d_type) => d_type,
        _ => panic!("Failed to parse type annotation: not a data type?"),
    };

    let type_annotation = TypeAnnotation {
        prefix: colon,
        data_type: data_type.clone(),
    };

    let tokens = tokens.map(|t| t.clone()).collect();

    (type_annotation, tokens)
}

fn parse_expression(tokens: Vec<Token>) -> (SyntaxTree, Vec<Token>) {
    let (literal, tokens) = parse_literal(tokens);

    let literal = match literal {
        SyntaxTree::Literal(lit) => lit,
        _ => panic!(),
    };
    let tree = SyntaxTree::Expression(Expression::Literal(literal));

    (tree, tokens)
}

/// Takes a collection of tokens and produces a literal tree.
fn parse_literal(tokens: Vec<Token>) -> (SyntaxTree, Vec<Token>) {
    let mut tokens = tokens.iter();

    let token = tokens.next().expect("Empty token stream?");
    let lit = match token {
        Token::Literal(lit) => lit.clone(),
        _ => panic!("Expected literal, found {token:?}"),
    };

    let tree = SyntaxTree::Literal(lit);
    let tokens = tokens.map(|t| t.clone()).collect();

    (tree, tokens)
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokens::{DataType, Identifier, Keyword, Literal, Special, Token};

    use super::{Expression, Statement, SyntaxTree};

    #[test]
    fn parse() {
        let tokens = vec![
            Token::Keyword(Keyword::Let),
            Token::Identifier(Identifier("code_name".to_string())),
            Token::Special(Special::Colon),
            Token::DataType(DataType::Int),
            Token::Special(Special::Assignment),
            Token::Literal(Literal::Int(3)),
            Token::Special(Special::StatementStop),
        ];

        let tree = super::parse(tokens);

        let expected = SyntaxTree::Statement(Statement::Declaration {
            keyword: Keyword::Let,
            identifier: Identifier("code_name".to_string()),
            type_annotation: super::TypeAnnotation {
                prefix: Special::Colon,
                data_type: DataType::Int,
            },
            expression: Expression::Literal(Literal::Int(3)),
        });

        assert_eq!(tree, expected);
    }

    #[test]
    fn parse_declaration() {
        let tokens = vec![
            Token::Keyword(Keyword::Let),
            Token::Identifier(Identifier("code_name".to_string())),
            Token::Special(Special::Colon),
            Token::DataType(DataType::Int),
            Token::Special(Special::Assignment),
            Token::Literal(Literal::Int(3)),
            Token::Special(Special::StatementStop),
        ];

        let (tree, _) = super::parse_declaration(tokens);

        let expected = SyntaxTree::Statement(Statement::Declaration {
            keyword: Keyword::Let,
            identifier: Identifier("code_name".to_string()),
            type_annotation: super::TypeAnnotation {
                prefix: Special::Colon,
                data_type: DataType::Int,
            },
            expression: Expression::Literal(Literal::Int(3)),
        });

        assert_eq!(tree, expected);
    }

    #[test]
    fn parse_expression() {
        let tokens = vec![Token::Literal(Literal::Int(314))];
        let (tree, _) = super::parse_expression(tokens);

        let expected = super::SyntaxTree::Expression(Expression::Literal(Literal::Int(314)));

        assert_eq!(tree, expected);

        let tokens = vec![Token::Literal(Literal::Int(314))];
        let (tree, _) = super::parse_expression(tokens);

        let expected = super::SyntaxTree::Expression(Expression::Literal(Literal::Int(314)));

        assert_eq!(tree, expected);
    }

    #[test]
    fn parse_literal() {
        use crate::lexer::tokens::{Literal, Token};

        let tokens = vec![Token::Literal(Literal::Int(314))];
        let (tree, _) = super::parse_literal(tokens);

        let expected = super::SyntaxTree::Literal(Literal::Int(314));

        assert_eq!(tree, expected);

        let tokens = vec![Token::Literal(Literal::Float(3.14))];
        let (tree, _) = super::parse_literal(tokens);

        let expected = super::SyntaxTree::Literal(Literal::Float(3.14));

        assert_eq!(tree, expected);
    }
}
