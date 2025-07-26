/*
grammar

program -> {
    declaration*,
    Eof
}


declaration -> {
    letDecl |
    funcDecl |
    statement
}

statement -> {
    exprStmt |
    echoStmt |
    block |
    ifStmt |
    whileStmt |
    forStmt |
    breakStmt
}

breakStmt -> {
    "break" ";"
}

forStmt -> {
    "for" "("
    ( letDecl | exprStmt | ";") 
    expression? ";"
    expression? ")"
    statement
}

whileStmt -> {
    "while" "(" expression ")" statement
}

ifStmt -> {
    "if" "(" expression ")" statement ("else" statement)?
}

block -> {
    "{" declaration* "}"

exprStmt -> {
    expression ";"
}

echoStmt -> {
    "echo" expression ";"
}

funcDecl -> {
    "fn" function
}

function -> {
    IDENTIFIER "(" parametrs? ")"
    block
}

letDecl -> {
    "let" IDENTIFIER ("=" expression)? ";"
}

expression -> {
    function_expression | assignment
}

function_expression -> {
}

assignment -> {
    IDENTIFIER "=" assignment | logic_or
}

logic_or -> {
    logic_and ("or" logic_and)*
}

logic_and -> {
    equality ("and" equality)*
}

literal -> {
    NUMBER | STRING |
    "true" | "false" | "null"
}

primary -> {
    "true" | "false" | "null" |
    NUMBER | STRING |
    "(" expression ")" |
    IDENTIFIER
}

grouping -> {
    "(" expression ")"
}

unary -> {
    ("-" | "!") unary | call
}

call -> {
    primary ( "(" arguments? ")" )*
}

arguments -> {
    expression ("," expression)*
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
use crate::stmt::Stmt;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug)]
enum FunctionType {
    Function
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        let mut errs = Vec::new();

        while !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(s) => stmts.push(s),
                Err(msg) => {
                    errs.push(msg);
                    self.synchronize();
                },
            }
        }

        if errs.len() == 0 {
            Ok(stmts)
        } else {
            Err(errs.join("\n"))
        }
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(TokenType::Let) {
            self.let_declaration()
        } else if self.match_token(TokenType::Fn) {
            self.function(FunctionType::Function)
        } else {
            self.statement()
        }
    }

    fn function(&mut self, type_: FunctionType) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, &format!("Expected {:?} after name", type_))?;

        self.consume(TokenType::LeftParen, &format!("Expected '(' after {:?} name", type_))?;
        
        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() > 255 {
                    return Err(format!("line: {}, Can not have more than 255 function arguments", self.peek().line_number));
                }

                let param = self.consume(TokenType::Identifier, "Expected parameter after name")?;
                parameters.push(param);

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

        self.consume(TokenType::LeftBrace, &format!("Expected '{{' before {:?} body", type_))?;
        let body = match self.block_statement()? {
            Stmt::Block { statements } => statements,
            _ => panic!("Block statement parsed something that was not a block")
        };

        Ok(Stmt::Function { name: name, params: parameters, body: body })
    }

    fn let_declaration(&mut self) -> Result<Stmt, String> {
        let token = self.consume(TokenType::Identifier, "Expected variable name")?;

        let initializer;
        if self.match_token(TokenType::Equal) {
            initializer = self.expression()?;
        } else {
            initializer = Expr::Literal { value: LiteralValue::Null};
        }
        
        self.consume(TokenType::Semicolon, "Expected ';' after variable declaration")?;
        Ok(Stmt::Let { name: token, initializer: initializer})
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(TokenType::Echo) {
            self.echo_statement()
        } else if self.match_token(TokenType::LeftBrace) {
            self.block_statement()
        } else if self.match_token(TokenType::If) {
            self.if_statement()
        } else if self.match_token(TokenType::While) {
            self.while_statement()
        } else if self.match_token(TokenType::For) {
            self.for_statement()
        } else if self.match_token(TokenType::Break) {
            self.break_statement()
        } else if self.match_token(TokenType::Return) {
            self.return_statement()
        } else {
            self.expression_statement()
        }
    }

    fn return_statement(&mut self) -> Result<Stmt, String> {
        let keyword = self.previous();
        let value;
        if !self.check(TokenType::Semicolon) {
            value = Some(self.expression()?);
        } else {
            value = None;
        }
        self.consume(TokenType::Semicolon, "Expected ';' after return value")?;

        Ok(Stmt::Return {
            keyword: keyword,
            value: value
        })
    }

    fn break_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::Semicolon, "Expected ';' after break statement")?;
        Ok(Stmt::Break)
    }

    fn for_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expected '(' after for")?;
        
        let initializer;
        if self.match_token(TokenType::Semicolon) {
            initializer = None;
        } else if self.match_token(TokenType::Let) {
            let let_decl = self.let_declaration()?;
            initializer = Some(let_decl);
        } else {
            let expr = self.expression_statement()?;
            initializer = Some(expr);
        }

        let condition;
        if !self.check(TokenType::Semicolon) {
            let expr = self.expression()?;
            condition = Some(expr);
        } else {
            condition = None;
        }

        self.consume(TokenType::Semicolon, "Expected ';' after for loop condition")?;

        let incrementer;
        if !self.check(TokenType::RightParen) {
            let expr = self.expression()?;
            incrementer = Some(expr);
        } else {
            incrementer = None;
        }
        
        self.consume(TokenType::RightParen, "Expected ')' after for loop clauses")?;

        let mut body = self.statement()?;

        if let Some(incr) = incrementer {
            body = Stmt::Block {
                statements: vec![
                    Box::new(body),
                    Box::new(Stmt::Expression { expression: incr })
                ]
            };
        }

        let cond;
        match condition {
            None => cond = Expr::Literal { value: LiteralValue::True },
            Some(c) => cond = c,
        }
        body = Stmt::While {
            condition: cond,
            body: Box::new(body),
        };

        if let Some(init) = initializer {
            body = Stmt::Block {
                statements: vec![Box::new(init), Box::new(body)]
            };
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result< Stmt, String> {
        self.consume(TokenType::LeftParen, "Expected '(' after while")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after while loop condition")?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::While { condition: condition, body: body})
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'")?;
        let predicate = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after if predicate")?;

        let then = Box::new(self.statement()?);

        let otherwise = if self.match_token(TokenType::Else) {
            let stmt = self.statement()?;
            Some(Box::new(stmt))
        } else {
            None
        };

        Ok(Stmt::If { predicate: predicate, then: then, otherwise: otherwise})
    }

    fn block_statement(&mut self) -> Result<Stmt, String> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let decl = self.declaration()?;
            statements.push(Box::new(decl));
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block")?;
        Ok(Stmt::Block { statements })
    }

    fn echo_statement(&mut self) -> Result<Stmt, String> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value")?;
        Ok(Stmt::Echo { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn expression(&mut self) -> Result<Expr, String> {
        /*if self.match_token(TokenType::Fn) {
            self.function_expression()
        } else {
            self.assignment()
        }*/
        self.assignment()
    }

    fn function_expression(&mut self) -> Result<Expr, String> {
        let paren = self.consume(TokenType::LeftParen, "Expected '(' after anonymous function")?;
        
        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() > 255 {
                    return Err(format!("line: {}, Can not have more than 255 function arguments", self.peek().line_number));
                }

                let param = self.consume(TokenType::Identifier, "Expected parameter after name")?;
                parameters.push(param);

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expected ')' after anonymous function parameters")?;
        
        self.consume(TokenType::LeftBrace, "Expected '{' after anonymous function declaration")?;
        let body = match self.block_statement()? {
            Stmt::Block { statements } => statements,
            _ => panic!("Block statement parsed somoething that was not a block")
        };

        Ok(Expr::AnonFunction {
            paren: paren,
            arguments: parameters,
            body: body
        })
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.or()?;

        if self.match_token(TokenType::Equal) {
            let value = self.expression()?;

            match expr {
                Expr::Variable { name } => Ok(Expr::Assign { name: name, value: Box::from(value) }),
                _ => Err(format!("Invalid assingment target"))
            }
        } else {
            return Ok(expr);
        }
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;

        while self.match_token(TokenType::Or) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical { left: Box::from(expr), operator, right: Box::from(right) };
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;

        while self.match_token(TokenType::And) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical { left: Box::from(expr), operator: operator, right:Box::from(right) };
        }

        Ok(expr)
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
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = Vec::new();
        
        if !self.check(TokenType::RightParen) {
            loop {
                let arg = self.expression()?;
                arguments.push(arg);
                if arguments.len() >= 255 {
                    let location = self.peek().line_number;
                    return Err(format!("line: {}, Can not have more than 255 function arguments", location));
                }

                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RightParen, "Expected ')' after arguments")?;

        Ok(Expr::Call { callee: Box::new(callee), paren: paren, arguments: arguments })
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
            TokenType::Identifier => {
                self.advance();
                result = Expr::Variable { name: self.previous() };
            }
            TokenType::Fn => {
                self.advance();
                result = self.function_expression()?;
            },
            _ => {
                return Err(String::from("Expected expression"));
            },
        }

        //self.advance();

        Ok(result)
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            let token = self.previous();
            Ok(token)
        } else {
            Err(String::from(msg))
        }
    }

    fn check(&mut self, type_: TokenType) -> bool {
        self.peek().token_type == type_
    }

    fn match_token(&mut self, type_: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.peek().token_type == type_ {
                self.advance();
                true
            } else {
                false
            }
        }
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for type_ in types {
            if self.match_token(*type_) {
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
                TokenType::Echo | TokenType::Return => return,
                _ => (),
            }
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Tokenizer;

    #[test]
    fn test_equality_with_paren() {
        let src = "1 == (2 + 3);";
        
        let mut tokenizer = Tokenizer::new(src);
        
        let tokens = tokenizer.tokenize().unwrap();
        
        let mut parser = Parser::new(tokens);
        
        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr[0].to_string();

        assert_eq!(string_expr, "(== 1 (group (+ 2 3)))")
    }
}