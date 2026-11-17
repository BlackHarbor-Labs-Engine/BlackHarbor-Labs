// Abstract Syntax Tree definitions for HarborLang

#[derive(Debug, Clone)]
pub enum Expression {
    Number(i64),
    String(String),
    Identifier(String),
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    LetStatement(String, Expression),
    ReturnStatement(Expression),
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
