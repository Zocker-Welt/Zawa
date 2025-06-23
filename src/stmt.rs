use crate::expr::Expr;
use crate::tokenizer::Token;

pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Let { name: Token, initializer: Expr },
}
