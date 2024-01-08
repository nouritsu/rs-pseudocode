use super::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Declare(String, String),
    Assign(String, Expr),
    Output(Vec<Expr>),
    Input(String),
}
