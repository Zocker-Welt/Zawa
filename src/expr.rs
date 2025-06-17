use crate::tokenizer::{Token, TokenType};

use crate::tokenizer;

#[derive(Debug, Clone)]
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
        if b { LiteralValue::True } else { LiteralValue::False }
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
                    (LiteralValue::Number(x), TokenType::Minus) => Ok(LiteralValue::Number(-x)),
                    (_, TokenType::Minus) => Err(format!("Not implemented for {}", right.to_string())),
                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    _ => todo!()
                }
            },
            Expr::Binary { left, operator, right} => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;

                match (&left, operator.token_type, &right) {
                    // Valid: Number + - * / Number
                    (LiteralValue::Number(x), TokenType::Plus, LiteralValue::Number(y)) => Ok(LiteralValue::Number(x + y)), // x + y
                    (LiteralValue::Number(x), TokenType::Minus, LiteralValue::Number(y)) => Ok(LiteralValue::Number(x - y)), // x - y
                    (LiteralValue::Number(x), TokenType::Star, LiteralValue::Number(y)) => Ok(LiteralValue::Number(x * y)), // x * y
                    (LiteralValue::Number(x), TokenType::Slash, LiteralValue::Number(y)) => Ok(LiteralValue::Number(x / y)), // x / y

                    // Valid: Number < > <= >= == != String
                    (LiteralValue::Number(x), TokenType::Less, LiteralValue::Number(y)) => Ok(LiteralValue::from_bool(x < y)), // x < y
                    (LiteralValue::Number(x), TokenType::Greater, LiteralValue::Number(y)) => Ok(LiteralValue::from_bool(x > y)), // x > y
                    (LiteralValue::Number(x), TokenType::LessEqual, LiteralValue::Number(y)) => Ok(LiteralValue::from_bool(x <= y)), // x <= y
                    (LiteralValue::Number(x), TokenType::GreaterEqual, LiteralValue::Number(y)) => Ok(LiteralValue::from_bool(x >= y)), // x >= y
                    (LiteralValue::Number(x), TokenType::EqualEqual, LiteralValue::Number(y)) => Ok(LiteralValue::from_bool(x == y)), // x == y
                    (LiteralValue::Number(x), TokenType::BangEqual, LiteralValue::Number(y)) => Ok(LiteralValue::from_bool(x != y)), // x != y

                    // Invalid: Number + - * / < > <= >= == != String
                    // Invalid: String + - * / < > <= >= == != Number
                    (LiteralValue::StringValue(s), operator, LiteralValue::Number(x)) => Err(format!("Invalid binary operator: {} for string: \"{}\", number: {}", operator, s, x)), // s op x
                    (LiteralValue::Number(x), operator, LiteralValue::StringValue(s)) => Err(format!("Invalid binary operator: {} for number: {}, string: \"{}\"", operator, x, s)), // x op s
                    
                    // Invalid: String - * / String
                    (LiteralValue::StringValue(s1), TokenType::Minus, LiteralValue::StringValue(s2)) => Err(format!("Invalid binary operator: Minus for string: \"{}\", string: \"{}\"", s1, s2)), // s1 - s2
                    (LiteralValue::StringValue(s1), TokenType::Star, LiteralValue::StringValue(s2)) => Err(format!("Invalid binary operator: Star for string: \"{}\", string: \"{}\"", s1, s2)), // s1 * s2
                    (LiteralValue::StringValue(s1), TokenType::Slash, LiteralValue::StringValue(s2)) => Err(format!("Invalid binary operator: Slash for string: \"{}\", string: \"{}\"", s1, s2)), // s1 / s2

                    // Valid: String + String
                    (LiteralValue::StringValue(s1), TokenType::Plus, LiteralValue::StringValue(s2)) => Ok(LiteralValue::StringValue(format!("{}{}", s1, s2))), // s1 + s2
                    
                    // Valid: String < > <= >= == != String
                    (LiteralValue::StringValue(s1), TokenType::Less, LiteralValue::StringValue(s2)) => Ok(LiteralValue::from_bool(s1 < s2)), // s1 < s2
                    (LiteralValue::StringValue(s1), TokenType::Greater, LiteralValue::StringValue(s2)) => Ok(LiteralValue::from_bool(s1 > s2)), // s1 > s2
                    (LiteralValue::StringValue(s1), TokenType::LessEqual, LiteralValue::StringValue(s2)) => Ok(LiteralValue::from_bool(s1 <= s2)), // s1 <= s2
                    (LiteralValue::StringValue(s1), TokenType::GreaterEqual, LiteralValue::StringValue(s2)) => Ok(LiteralValue::from_bool(s1 >= s2)), // s1 >= s2
                    (LiteralValue::StringValue(s1), TokenType::EqualEqual, LiteralValue::StringValue(s2)) => Ok(LiteralValue::from_bool(s1 == s2)), // s1 == s2
                    (LiteralValue::StringValue(s1), TokenType::BangEqual, LiteralValue::StringValue(s2)) => Ok(LiteralValue::from_bool(s1 != s2)), // s1 != s2

                    _ => todo!()
                }
            },
            _ => todo!()
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