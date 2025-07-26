use crate::expr::Expr;
use crate::tokenizer::Token;

#[derive(Clone)]
pub enum Stmt {
    Expression { expression: Expr },
    Echo { expression: Expr },
    Let { name: Token, initializer: Expr },
    Block { statements: Vec<Box<Stmt>> },
    If { predicate: Expr, then: Box<Stmt>, otherwise: Option<Box<Stmt>> },
    While { condition: Expr, body: Box<Stmt> },
    Break,
    Function { name: Token, params: Vec<Token>, body: Vec<Box<Stmt>> },
    Return { keyword: Token, value: Option<Expr> },
}

impl Stmt {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Stmt::Expression { expression } => expression.to_string(),
            Stmt::Echo { expression } => format!(
                "(print {})",
                expression.to_string()
            ),
            Stmt::Let { name, initializer: _ } => format!(
                "(let {})",
                name.lexeme
            ),
            Stmt::Block { statements } => format!(
                "(block {})",
                statements.into_iter().map(|stmt| stmt.to_string()).collect::<String>()
            ),
            Stmt::If { predicate: _, then: _, otherwise: _ } => todo!(),
            Stmt::While { condition: _, body: _ } => todo!(),
            Stmt::Break => String::from("(break)"),
            Stmt::Function { name: _, params: _, body: _ } => todo!(),
            Stmt::Return { keyword: _, value: _ } => todo!(),
        }
    }
}