/*
grammar

expression -> {
    literal,
    unary,
    binary,
    grouping
}

literal -> {
    NUMBER, STRING,
    "true", "false", "null"
}

grouping -> {
    "(" expression ")"
}
unary -> {
    ("-", "!") expression
}

binary -> {
    expression operator expression
}

operator -> {
    "==", "!=", "<=", ">=", "<", ">",
    "+", "-", "*", "/"
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

    pub fn expression(self: &mut Self) -> Expr {
        self.equality()
    }

    fn equality(self: &mut Self) -> Expr {
        let mut expr = self.comparison();
        let mut matches_eq = self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]);
        while matches_eq {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: operator,
                right: Box::from(right)
            };

            matches_eq = self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]);
        }

        expr
    }

    fn comparison(self: &mut Self) -> Expr {
        let mut expr = self.term();

        while self.match_tokens(&[TokenType::Greater, TokenType::Less, TokenType::LessEqual]) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(right)
            }
        }

        expr
    }

    fn term(self: &mut Self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(right)
            }
        }

        expr
    }

    fn factor(self: &mut Self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(right)
            }
        }

        expr
    }

    fn unary(self: &mut Self) -> Expr {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary();
            Expr::Unary {
                operator: op,
                right: Box::from(right)
            }
        } else {
            self.primary()
        }
    }

    fn primary(self: &mut Self) -> Expr {
        if self.match_token(&TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expected ')'");
            Expr::Grouping {
                expression: Box::from(expr)
            }
        } else {
            let token = self.peek();
            self.advance();
            Expr::Literal {
                value: LiteralValue::from_token(token.clone())
            }
        }
    }

    fn consume(self: &mut Self, token_type: TokenType, msg: &str) {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
        } else {
            panic!("{}", msg);
        }
    }

    fn match_token(self: &mut Self, type_: &TokenType) -> bool {
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

    fn match_tokens(self: &mut Self, types: &[TokenType]) -> bool {
        for type_ in types {
            if self.match_token(type_) {
                return true;
            }
        }

        false
    }

    fn advance(self: &mut Self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn peek(self: &mut Self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(self: &mut Self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(self: &mut Self) -> bool {
        self.peek().token_type == TokenType::Eof
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
        
        let parsed_expr = parser.expression();
        let string_expr = parsed_expr.to_string();

        assert_eq!(string_expr, "(+ 1 2)");
    }

    #[test]
    fn test_comparison() {
        let src = "1 + 2 == 5 + 7";
        
        let mut tokenizer = Tokenizer::new(src);
        
        let tokens = tokenizer.scan_tokens().unwrap();
        
        let mut parser = Parser::new(tokens);
        
        let parsed_expr = parser.expression();
        let string_expr = parsed_expr.to_string();

        assert_eq!(string_expr, "(== (+ 1 2) (+ 5 7))")
    }
}