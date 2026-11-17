use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LexerError {
    UnexpectedChar(char),
    UnterminatedStringLiteral,
    InvalidNumberFormat,
    InvalidIdentifier,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i64),
    StringLiteral(String),
    Keyword(Keyword),
    Operator(Operator),
    Delimiter(Delimiter),
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Keyword {
    Let,
    Fn,
    If,
    Else,
    While,
    Return,
    True,
    False,
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Delimiter {
    LParen, // ( 
    RParen, // ) 
    LBrace, // { 
    RBrace, // } 
    Semicolon, // ; 
    Comma, // , 
}
