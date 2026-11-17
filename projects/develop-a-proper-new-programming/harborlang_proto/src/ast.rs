<file path="harborlang_proto/src/ast.rs">
use std::fmt;

/// A clean, production-ready Algebraic Data Type (ADT) structure for the HarborLang AST.
/// This structure is designed to be robust, explicit, and ready for parsing output from `syntax.rs`.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableRef(Identifier);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionCall(Identifier, Vec<Expr>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Not,
    Assign,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Expr {
    Literal(Literal),
    Variable(VariableRef),
    Binary(Box<Expr>, Operator, Box<Expr>),
    FunctionCall(FunctionCall),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    While(Box<Expr>, Box<Stmt>),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Stmt {
    Expr(Expr),
    Assignment(Identifier, Expr),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    While(Box<Expr>, Box<Stmt>),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
}
</file>
