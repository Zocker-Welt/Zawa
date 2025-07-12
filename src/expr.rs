// 12 43

/*
For future:

make so in this case it will write String("a")
>>> true * "a"
Binary operator Star cannot be applied for operands True, StringValue("a")
*/

use crate::tokenizer::{Token, TokenType};
use crate::tokenizer;
use crate::environment::Environment;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub enum LiteralValue {
    Number(f64),
    StringValue(String),
    True,
    False,
    Null,
    Callable { 
        name: String,
        arity: usize,
        fn_: Rc<dyn Fn(&Vec<LiteralValue>) -> LiteralValue>,
    },
}
use LiteralValue::*;

impl std::fmt::Debug for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Number(x), Number(y)) => x == y,
            (
                Callable {
                    name,
                    arity,
                    fn_: _
                },
                Callable {
                    name: name2,
                    arity: arity2,
                    fn_: _
                }
            ) => name == name2 && arity == arity2,
            (StringValue(s1), StringValue(s2)) => s1 == s2,
            (True, True) => true,
            (False, False) => true,
            (Null, Null) => true,
            _ => false,
        }
    }
}

fn unwrap_as_f64(literal: Option<tokenizer::LiteralValue>) -> f64 {
    match literal {
        Some(tokenizer::LiteralValue::FValue(x)) => x as f64,
        _ => panic!("Could not unwrap as f64")
    }
}

fn unwrap_as_string(literal: Option<tokenizer::LiteralValue>) -> String {
    match literal {
        Some(tokenizer::LiteralValue::StringValue(s)) => s.clone(),
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
            Null => String::from("null"),
            Callable { name, arity , fn_: _ } => format!("{}/{}", name, arity)
        }
    }

    pub fn to_type(&self) -> &str {
        match self {
            Number(_) => "Number",
            StringValue(_) => "String",
            True => "Boolean",
            False => "Boolean",
            Null => "Null",
            Callable { name: _, arity: _ , fn_: _ } => "Callable"
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(unwrap_as_f64(token.literal)),
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
            Self::Null => Self::True,
            Self::Callable { name: _, arity: _ , fn_: _ } => Self::False
        }
    }

    pub fn is_truthy(&self) -> LiteralValue {
        match self {
            Self::Number(x) => if *x == 0.0 { Self::False } else { Self::True },
            Self::StringValue(s) => if s.len() == 0 { Self::False } else { Self::True },
            Self::True => Self::True,
            Self::False => Self::False,
            Self::Null => Self::False,
            Self::Callable { name: _, arity: _, fn_: _ } => Self::True
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
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Literal {
        value: LiteralValue
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    },
    Variable {
        name: Token
    },
    Assign {
        name: Token,
        value: Box<Expr>
    },
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Expr {
    #[allow(dead_code)]
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
            Expr::Call { callee, paren: _, arguments} => format!(
                "({} {:?})",
                (*callee).to_string(),
                arguments
            ),
            Expr::Grouping { expression } => format!(
                "(group {})",
                expression.to_string()
            ),
            Expr::Literal { value } => format!(
                "{}",
                value.to_string()
            ),
            Expr::Logical { left, operator, right } => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string()
            ),
            Expr::Unary { operator, right} => {
                let operator_str = operator.lexeme.clone();
                let right_str = right.to_string();
                format!("({} {})", operator_str, right_str)
            },
            Expr::Variable { name } => format!(
                "var {}",
                name.lexeme
            ),
            Expr::Assign { name, value } => format!(
                "{} = {}",
                name.lexeme, value.to_string()
            ),
        }
    }

    pub fn evaluate(&self, environment: Rc<RefCell<Environment>>) -> Result<LiteralValue, String> {
        match self {
            Expr::Assign { name, value } => {
                let new_value = (*value).evaluate(environment.clone())?;
                let assign_success = environment.borrow_mut().assign(&name.lexeme, new_value.clone());

                if assign_success {
                    Ok(new_value)
                } else {
                    Err(format!("{} was not declared in this scope", name.lexeme))
                }
            },
            Expr::Call { callee, paren, arguments } => {
                let callable = (*callee).evaluate(environment.clone())?;
                match callable {
                    Callable { name, arity, fn_} => {
                        if arguments.len() != arity {
                            return Err(format!(
                                "Callable {} expected {} arguments but {} were given",
                                name,
                                arity,
                                arguments.len()
                            ));
                        }

                        let mut arg_vals = Vec::new();
                        for arg in arguments {
                            let val = arg.evaluate(environment.clone())?;
                            arg_vals.push(val);
                        }
                        
                        Ok(fn_(&arg_vals))
                    },
                    other => Err(format!("{} is not callable", other.to_string()))
                }
            },
            Expr::Variable { name } => {
                match environment.borrow().get(&name.lexeme) {
                    Some(value) => Ok(value.clone()),
                    None => Err(format!("{} was not declared in this scope", name.lexeme))
                }
            },
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Logical { left, operator, right } => {
                match operator.token_type {
                    TokenType::Or => {
                        let lhs_value = left.evaluate(environment.clone())?;
                        let lhs_true = lhs_value.is_truthy();
                        if lhs_true == True {
                            return Ok(lhs_value);
                        } else {
                            return right.evaluate(environment.clone());
                        }
                    },
                    TokenType::And => {
                        let lhs_value = left.evaluate(environment.clone())?;
                        let lhs_true = lhs_value.is_truthy();
                        if lhs_true == False {
                            return Ok(lhs_value);
                        } else {
                            return right.evaluate(environment.clone());
                        }
                    },
                    token_type => return Err(format!("Invalid token in logical expression: {}", token_type)),
                }
            },
            Expr::Grouping { expression } => expression.evaluate(environment.clone()),
            Expr::Unary {operator, right} => {
                let right = right.evaluate(environment.clone())?;

                match (&right, operator.token_type) {
                    (Number(x), TokenType::Minus) => Ok(Number(-x)),
                    (_, TokenType::Minus) => Err(format!("Minus not implemented for {}", right.to_type())),
                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    (_, token_type) => Err(format!("{} is not a valid unary operator", token_type))
                }
            },
            Expr::Binary { left, operator, right} => {
                let left = left.evaluate(environment.clone())?;
                let right = right.evaluate(environment.clone())?;

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

    #[allow(dead_code)]
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

        let mul = Token {
            token_type: TokenType::Star,
            lexeme: String::from("*"),
            literal: None,
            line_number: 0
        };

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