/*

--- Expr still needs to be tested using #[cfg(test)]

*/

use crate::tokenizer::{Token, TokenType};

pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Null,
}
use LiteralValue::*;

impl LiteralValue {
    pub fn to_string(self: &Self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(s) => s.clone(),
            LiteralValue::True => String::from("true"),
            LiteralValue::False => String::from("false"),
            LiteralValue::Null => String::from("null"),
        }
    }
}

pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValue },
    Unary { operator: Token, right: Box<Expr> },
}

impl Expr {
    pub fn to_string(self: &Self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right
            }
            => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string()
            ),
            Expr::Grouping { expression } => format!("(group {})", (*expression).to_string()),
            Expr::Literal {value } => format!("{}", value.to_string()),
            Expr::Unary { operator, right } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({} {})", operator_str, right_str)
            }
        }
    }

    pub fn print(self: &Self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_print_ast() {
        let minus_token = Token::new(
            TokenType::Minus,
            String::from("-"),
            None,
            0
        );

        let ott = Expr::Literal { // 123
            value: Number(123.0)
        };

        let group = Expr::Grouping {
            expression: Box::from(Expr::Literal {
                value: Number(45.67)
            })
        };

        let mul = Token::new(
            TokenType::Star,
            String::from("*"),
            None,
            0
        );

        let ast = Expr::Binary {
            left: Box::from(Expr::Unary {
                operator: minus_token,
                right: Box::from(ott)
            }),
            operator: mul,
            right: Box::from(group)
        };

        let result = ast.to_string();
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
