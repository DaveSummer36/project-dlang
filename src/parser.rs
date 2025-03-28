use crate::lexer::{ Lexer, Token };
use std::iter::Peekable;

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Let {
        name: String,
        type_annot: Option<String>,
        value: Expr,
    },
    Return(Option<Expr>),
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Option<String>)>,
    pub return_type: Option<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    current_token: Option<Token, (usize, usize)>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut lexer = lexer.peekable();
        let current_token = lexer.next();
        Self { lexer, current_token }
    }

    fn peek_token(&mut self) -> Option<&Token> {
        self.lexer.peek().map(|(token, _)| token)
    }

    fn consume_token(&mut self) {
        self.current_token = self.lexer.next();
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), String> {
        match &self.current_token {
            Some((Token, _)) if *token == expected => {
                self.consume_token();
                Ok(())
            }
            Some((_, span)) =>
                Err(
                    format!(
                        "Expected{:?}, found {:?} at position {:?}",
                        expected,
                        self.current_token,
                        span
                    )
                ),
            None => Err(format!("Expected {:?}, but reached end of input", expected)),
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut functions = Vec::new();

        while self.current_token.is_some() {
            if let Some((Token::KeywordFn, _)) = &self.current_token() {
                functions.push(self.parse_function()?);
            } else {
                return Err(
                    format!("Expected function declaration, found {:?}", self.current_token)
                );
            }
        }

        Ok(Program { functions })
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        self.expect_token(Token::KeywordFn)?;

        let name = match self.current_token.clone() {
            Some((Token::Ident(name), _)) => name,
            _ => {
                return Err("Expected function name".to_string());
            }
        };
        self.consume_token();

        self.expect_token(Token::LParen)?;
        let params = self.parse_params()?;
        self.expect_token(Token::RParen)?;

        let return_type = if let Some(Token::Arrow) = self.peek_token() {
            self.consume_token();
            self.parse_type_annotation()?
        } else {
            None
        };

        self.expect_token(Token::LBrace)?;
        let body = self.parse_block()?;
        self.expect_token(Token::RBrace)?;

        Ok(Function { name, params, return_type, body })
    }

    fn parse_params(&mut self) -> Result<Vec<(String, Option<String>)>, String> {
        let mut params = Vec::new();

        while let Some((Token::Ident(name), _)) = self.current_token.clone() {
            self.consume_token();

            let type_annot = if let Some(Token::Colon) = self.peek_token() {
                self.consume_token();
                Some(self.parse_type_annotation()?)
            } else {
                None
            };

            params.push((name, type_annot));

            if let Some(Token::Comma) = self.peek_token() {
                self.consume_token();
            } else {
                break;
            }

            Ok(params);
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();

        while self.current_token.is_some() && !matches!(self.peek_token(), Some(Token::RBrace)) {
            stmts.push(self.parse_stmt()?);

            if let Some(Token::Semicolon) = self.peek_token() {
                self.consume_token();
            }
        }

        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.current_token.clone() {
            Some((Token::KeywordLet, _)) => self.parse_let_stmt(),
            Some((Token::KeywordReturn, _)) => self.parse_return_stmt(),
            Some((Token::KeywordWhile, _)) => self.parse_while_stmt(),
            Some((Token::KeywordIf, _)) => self.parse_if_expr().map(Stmt::Expr),
            _ => self.parse_expr().map(Stmt::Expr),
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt, String> {
        self.expect_token(Token::KeywordLet)?;

        let name = match self.current_token.clone() {
            Some((Token::Ident(name), _)) => name,
            _ => {
                return Err("Expected variable name".to_string());
            }
        };
        self.consume_token();

        let type_annot = if let Some(Token::Colon) = self.peek_token() {
            self.consume_token();
        } else {
            None
        };

        self.expect_token(Token::Equals)?;
        let value = self.parse_expr()?;

        Ok(Stmt::Let {
            name,
            type_annot,
            value,
        })
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, String> {
        self.expect_token(Token::KeywordReturn)?;

        let expr = if !matches!(self.peek_token(), Some(Token::Semicolon)) {
            Some(self.parse_expr()?)
        } else {
            None
        };

        Ok(Stmt::Return(expr))
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt, String> {
        self.expect_token(Token::KeywordWhile)?;

        let condition = self.parse_expr()?;
        self.expect_token(Token::LBrace)?;
        let body = self.parse_block()?;
        self.expect_token(Token::RBrace)?;

        Ok(Stmt::While { condition, body })
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_binary_expr(0)
    }

    fn parse_binary_expr(&mut self, precedence: u8) -> Result<Expr, String> {
        let mut left = self.parse_primary_expr()?;

        while let Some(op) = self.current_binary_op() {
            let op_prec = self.op_precedence(&op);
            if op_prec < precedence {
                break;
            }

            self.consume_token();
            let right = self.parse_binary_expr(op_prec + 1)?;
            left = Expr::BinaryOp {
                left: Expr::BinaryOp,
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary_expr(&mut self) -> Result<Expr, String> {
        match self.current_token.clone() {
            Some((Token::Int(n), _)) => {
                self.consume_token();
                Ok(Expr::Literal(Literal::Int(n)))
            }

            Some((Token::Float(n), _)) => {
                self.consume_token();
                Ok(Expr::Literal(Literal::Float(n)))
            }

            Some((Token::StringLit(s), _)) => {
                self.consume_token();
                Ok(Expr::Literal(Literal::String(s)))
            }

            Some((Token::Bool(bool), _)) => {
                self.consume_token();
                Ok(Expr::Literal(Literal::Bool(b)))
            }

            Some((Token::Ident(name), _)) => {
                self.consume_token();
                if let Some(Token::LParen) = self.peek_token() {
                    self.parse_call_expr(name)
                } else {
                    Ok(Expr::Ident(name))
                }
            }

            Some((Token::LParen, _)) => {
                self.consume_token();
                let expr = self.parse_expr()?;
                self.expect_token(Token::RParen)?;
                Ok(expr)
            }

            Some((Token::KeywordIf, _)) => self.parse_if_expr(),
            _ => Err("Expected expression".to_string()),
        }
    }

    fn parse_call_expr(&mut self, callee: String) -> Result<Expr, String> {
        self.expect_token(Token::LParen)?;

        let mut args = Vec::new();
        while !matches!(self.peek_token(), Some(Token::RParen)) {
            args.push(self.parse_expr()?);

            if let Some(Token::Comma) = self.peek_token() {
                self.consume_token();
            } else {
                break;
            }
        }

        self.expect_token(Token::RParen)?;
        Ok(Expr::Call { callee, args })
    }

    fn parse_if_epxr(&mut self) -> Result<Expr, String> {
        self.expect_token(Token::KeywordIf)?;

        let condition = Box::new(self.parse_expr()?);
        self.expect_token(Token::LBrace)?;
        let then_branch = self.parse_block()?;
        self.expect_token(Token::RBrace)?;

        let else_branch = if let Some(Token::KeywordElse) = self.peek_token() {
            self.consume_token();
            self.expect_token(Token::LBrace)?;
            let else_branch = self.parse_block()?;
            self.expect_token(Token::RBrace)?;
            Some(else_branch)
        } else {
            None
        };

        Ok(Expr::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn current_binary_op(&mut self) -> Option<BinaryOp> {
        match self.peek_token()? {
            Token::Plus => Some(BinaryOp::Add),
            Token::Minus => Some(BinaryOp::Sub),
            Token::Star => Some(BinaryOp::Mul),
            Token::Slash => Some(BinaryOp::Div),
            Token::DoubleEquals => Some(BinaryOp::Eq),
            Token::NotEquals => Some(BinaryOp::Neq),
            Token::LessThan => Some(BinaryOp::Lt),
            Token::GreateThan => Some(BinaryOp::Gt),
            Token::LessOrEqual => Some(BinaryOp::Le),
            Token::GreaterOrEqual => Some(BinaryOp::Ge),
            Token::And => Some(BinaryOp::And),
            Token::Or => Some(BinaryOp::Or),
            _ => None,
        }
    }

    fn op_precedence(&self, op: &BinaryOp) -> u8 {
        match op {
            BinaryOp::Mul | BinaryOp::Div => 5,
            BinaryOp::Add | BinaryOp::Sub => 4,
            | BinaryOp::Eq
            | BinaryOp::Neq
            | BinaryOp::Lt
            | BinaryOp::Gt
            | BinaryOp::Le
            | BinaryOp::Ge => 3,
            BinaryOp::And => 2,
            BinaryOp::Or => 1,
        }
    }

    fn parse_type_annotation(&mut self) -> Result<String, String> {
        match self.current_token.clone() {
            Some((Token::Ident(ty), _)) => {
                self.consume_token();
                Ok(ty)
            }
            _ => Err("Expected type annotation".to_string()),
        }
    }
}
