use crate::expr::Expr;
use crate::tokenizer::Token;

pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Let { name: Token, initializer: Expr },
    Block { statements: Vec<Stmt> },
    If { predicate: Expr, then: Box<Stmt>, otherwise: Option<Box<Stmt>> },
}

impl Stmt {
    pub fn to_string(&self) -> String {
        match self {
            Stmt::Expression { expression } =>expression.to_string() ,
            Stmt::Print { expression } => format!("(print {})", expression.to_string()),
            Stmt::Let { name, initializer } => format!("(let {})", name.lexeme),
            Stmt::Block { statements } => format!(
                "(block {})",
                statements.into_iter().map(|stmt| stmt.to_string()).collect::<String>()
            ),
            Stmt::If { predicate, then, otherwise } => todo!()
        }
    }
}