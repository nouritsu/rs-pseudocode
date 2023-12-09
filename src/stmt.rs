use crate::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Declare(String, Expr),
    Assign(String, Expr),
    Output(Vec<Expr>),
    Input(String),
}
