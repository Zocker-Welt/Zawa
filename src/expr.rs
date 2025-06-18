// 12 43

/*
For future:

make so in this case it will write String("a")
>>> true * "a"
Binary operator Star cannot be applied for operands True, StringValue("a")
*/

use crate::tokenizer::{Token, TokenType};

use crate::tokenizer;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Null
}
use LiteralValue::*;

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
            Number(x) => x.to_string(),
            StringValue(s) => s.clone(),
            True => String::from("true"),
            False => String::from("false"),
            Null => String::from("null")
        }
    }

    pub fn to_type(&self) -> &str {
        match self {
            Number(_) => "Number",
            StringValue(_) => "String",
            True => "Boolean",
            False => "Boolean",
            Null => "Null"
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

    pub fn is_falsy(&self) -> LiteralValue {
        match self {
            Self::Number(x) => if *x == 0.0 { Self::True } else { Self::False },
            Self::StringValue(s) => if s.len() == 0 { Self::True } else { Self::False },
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Null => Self::True
        }
    }

    pub fn from_bool(b: bool) -> LiteralValue {
        if b { True } else { False }
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

    pub fn evaluate(&self) -> Result<LiteralValue, String> {
        match self {
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Grouping { expression } => expression.evaluate(),
            Expr::Unary {operator, right} => {
                let right = right.evaluate()?;

                match (&right, operator.token_type) {
                    (Number(x), TokenType::Minus) => Ok(Number(-x)),
                    (_, TokenType::Minus) => Err(format!("Minus not implemented for {}", right.to_type())),
                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    (_, token_type) => Err(format!("{} is not a valid unary operator", token_type))
                }
            },
            Expr::Binary { left, operator, right} => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;

                match (&left, operator.token_type, &right) {
                    //expreimental
                    //(Number(x), TokenType::Slash, Number(0.0)) => Err(format!("Binary operator Slash cannot be applied for operands {:?}, Number(0.0)", Number(*x))),

                    (Number(x), TokenType::Plus, Number(y)) => Ok(Number(x + y)),
                    (Number(x), TokenType::Minus, Number(y)) => Ok(Number(x - y)),
                    (Number(x), TokenType::Star, Number(y)) => Ok(Number(x * y)),
                    (Number(x), TokenType::Slash, Number(y)) => Ok(Number(x / y)),

                    (Number(x), TokenType::Greater, Number(y)) => Ok(LiteralValue::from_bool(x > y)),
                    (Number(x), TokenType::GreaterEqual, Number(y)) => Ok(LiteralValue::from_bool(x >= y)),
                    (Number(x), TokenType::Less, Number(y)) => Ok(LiteralValue::from_bool(x < y)),
                    (Number(x), TokenType::LessEqual, Number(y)) => Ok(LiteralValue::from_bool(x <= y)),

                    (StringValue(s), operator, Number(x)) => Err(format!("Binary operator {} cannot be applied for operands {:?}, {:?}", operator, StringValue(String::from(s)), Number(*x))),
                    (Number(x), operator, StringValue(s)) => Err(format!("Binary operator {} cannot be applied for operands {:?}, {:?}", operator, Number(*x), StringValue(String::from(s)))),

                    (StringValue(s1), TokenType::Plus, StringValue(s2)) => Ok(StringValue(format!("{}{}", s1, s2))),

                    (a, TokenType::BangEqual, b) => Ok(LiteralValue::from_bool(a != b)),
                    (a, TokenType::EqualEqual, b) => Ok(LiteralValue::from_bool(a == b)),

                    (StringValue(s1), TokenType::Greater, StringValue(s2)) => Ok(LiteralValue::from_bool(s1 > s2)),
                    (StringValue(s1), TokenType::GreaterEqual, StringValue(s2)) => Ok(LiteralValue::from_bool(s1 >= s2)),
                    (StringValue(s1), TokenType::Less, StringValue(s2)) => Ok(LiteralValue::from_bool(s1 < s2)),
                    (StringValue(s1), TokenType::LessEqual, StringValue(s2)) => Ok(LiteralValue::from_bool(s1 <= s2)),

                    (a, token_type, b) => Err(format!("Binary operator {} cannot be applied for operands {:?}, {:?}", token_type, a, b)),
                }
            }
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