use crate::tokenizer::{Token, TokenType};

use crate::tokenizer;

pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Null
}

fn unwrap_as_f32(literal: Option<tokenizer::LiteralValue>) -> f32 {
    match literal {
        Some(tokenizer::LiteralValue::IntValue(x)) => x as f32,
        Some(tokenizer::LiteralValue::FValue(x)) => x as f32,
        _ => panic!("Could not unwrap as f32")
    }
}

fn unwrap_as_string(literal: Option<tokenizer::LiteralValue>) -> String {
    match literal {
        Some(tokenizer::LiteralValue::StringValue(s)) => s.clone(),
        Some(tokenizer::LiteralValue::IdentifierVal(s)) => s.clone(),
        _ => panic!("Could not unwrap as string")
    }
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(s) => s.clone(),
            LiteralValue::True => String::from("true"),
            LiteralValue::False => String::from("false"),
            LiteralValue::Null => String::from("null")
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(unwrap_as_f32(token.literal)),
            TokenType::StringLit => Self::StringValue(unwrap_as_string(token.literal)),
            TokenType::True => Self::True,
            TokenType::False => Self::False,
            TokenType::Null => Self::Null,
            _ => panic!("Could not create LiteralValue from {:?}", token)
        }
    }
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Grouping {
        expression: Box<Expr>
    },
    Literal {
        value: LiteralValue
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    }
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right
            } => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string()
            ),
            Expr::Grouping { expression } => format!(
                "(group {})",
                expression.to_string()
            ),
            Expr::Literal { value } => format!(
                "{}",
                value.to_string()
            ),
            Expr::Unary { operator, right} => {
                let operator_str = operator.lexeme.clone();
                let right_str = right.to_string();
                format!("({} {})", operator_str, right_str)
            },
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_print_ast() {
        let minus_token = Token {
            token_type: TokenType::Minus,
            lexeme: String::from("-"),
            literal: None,
            line_number: 0
        };

        let ott = Expr::Literal { // 123
            value: LiteralValue::Number(123.0)
        };

        let group = Expr::Grouping {
            expression: Box::from(Expr::Literal {
                value: LiteralValue::Number(45.67)
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