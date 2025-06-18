// Only expects expressions - identifiers are not implemented yet

/*
grammar

expression -> {
    literal |
    unary |
    binary |
    grouping
}

literal -> {
    NUMBER | STRING,
    "true" | "false" | "null"
}

grouping -> {
    "(" expression ")"
}
unary -> {
    ("-" | "!") expression
}

binary -> {
    expression operator expression
}

operator -> {
    "==" | "!=" | "<=" | ">=" | "<" | ">" |
    "+" | "-" | "*" | "/"
}
*/

use crate::tokenizer::{TokenType, Token};
use crate::expr::{Expr, LiteralValue};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        let mut matches_eq = self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]);
        while matches_eq {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(right)
            };

            matches_eq = self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_tokens(&[TokenType::Greater, TokenType::GreaterEqual , TokenType::Less, TokenType::LessEqual]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(right)
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(right)
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(right)
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary {
                operator: op,
                right: Box::from(right)
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek();
        
        let result;
        match token.token_type {
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')'")?;
                result = Expr::Grouping {
                    expression: Box::from(expr)
                };
            },
            TokenType::True | TokenType::False | TokenType::Null |  TokenType::Number | TokenType::StringLit => {
                self.advance();
                result = Expr::Literal {
                    value: LiteralValue::from_token(token.clone())
                };
            },
            _ => return Err(String::from("Expected expression")),
        }

        //self.advance();

        Ok(result)
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            Ok(())
        } else {
            Err(String::from(msg))
        }
    }

    fn match_token(&mut self, type_: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.peek().token_type == *type_ {
                self.advance();
                true
            } else {
                false
            }
        }
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for type_ in types {
            if self.match_token(type_) {
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class | TokenType::Fn | TokenType::Let |
                TokenType::For | TokenType::If | TokenType::While |
                TokenType::Print | TokenType::Return => return,
                _ => (),
            }
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::{LiteralValue, Tokenizer};

    #[test]
    fn  test_addition() {
        let one = Token {
            token_type: TokenType::Number,
            lexeme: String::from("1"),
            literal: Some(LiteralValue::IntValue(1)),
            line_number: 0
        };
        let plus = Token {
            token_type: TokenType::Plus,
            lexeme: String::from("+"),
            literal: None,
            line_number: 0
        };
        let two = Token {
            token_type: TokenType::Number,
            lexeme: String::from("2"),
            literal: Some(LiteralValue::IntValue(2)),
            line_number: 0
        };
        let semi = Token {
            token_type: TokenType::Semicolon,
            lexeme: String::from(";"),
            literal: None,
            line_number: 0
        };

        let tokens = vec![one, plus, two, semi];
        let mut parser = Parser::new(tokens);
        
        let parsed_expr = parser.parse().unwrap(); // we dont check for the errors rn
        let string_expr = parsed_expr.to_string();

        assert_eq!(string_expr, "(+ 1 2)");
    }

    #[test]
    fn test_equality_with_paren() {
        let src = "1 == (2 + 3)";
        
        let mut tokenizer = Tokenizer::new(src);
        
        let tokens = tokenizer.tokenize().unwrap();
        
        let mut parser = Parser::new(tokens);
        
        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr.to_string();

        assert_eq!(string_expr, "(== 1 (group (+ 2 3)))")
    }
}
