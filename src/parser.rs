use crate::lexer::{Token, Span, LexerToken, Keyword, Symbol};
use crate::ast::*;

#[derive(Debug, Clone)]
pub enum ParserError {
    Unspecified(String, Span),
    NoMoreTokens,
    NoParseletFound(String, Token),
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

pub type ParserResult<T> = Result<T, ParserError>;

pub type PrefixParselet = Fn(&mut Parser, Token) -> ParserResult<Expr>;
pub type InfixParselet = Fn(&mut Parser, Token, Expr) -> ParserResult<Expr>;

fn new_object_prslt(p: &mut Parser, t: Token) -> ParserResult<Expr> {
    unimplemented!("NEWOBJ");
}

fn variable_reference_prslt(p: &mut Parser, t: Token) -> ParserResult<Expr> {
    Ok(Identifier::new_expr((t.0.as_identifier().unwrap(), t.1)))
}

fn binary_prslt(left: bool, prec: u32) -> (Box<InfixParselet>, u32) {
    (Box::new(|p: &mut Parser, t: Token, lhs: Expr| -> ParserResult<Expr> {
        let rhs = p.expression(if left { prec } else { prec - 1 })?;
        Ok(Binary::new_expr(lhs, rhs, t))
    }), prec)
}

// A top-down parser
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            index: 0,
        }
    }

    pub fn file(&mut self) -> ParserResult<Vec<Stat>> {
        unimplemented!()
    }

    fn get_prefix_parselet(tok: &Token) -> Option<Box<PrefixParselet>> {
        match tok {
            &(LexerToken::Keyword(Keyword::New), _) => Some(Box::new(new_object_prslt)),
            &(LexerToken::Identifier(_), _) => Some(Box::new(variable_reference_prslt)),
            _ => None
        }
    }

    fn get_infix_parselet(tok: &Token) -> Option<(Box<InfixParselet>, u32)> {
        match tok {
            &(LexerToken::Symbol(Symbol::Plus), _) => Some(binary_prslt(false, 10)),
            &(LexerToken::Symbol(Symbol::Asterisk), _) => Some(binary_prslt(false, 20)),
            _ => None
        }
    }

    pub fn expression(&mut self) -> ParserResult<Expr> {
        self.pratt_expression(0)
    }

    pub fn pratt_expression(&mut self, prec: u32) -> ParserResult<Expr> {
        let mut token = self.next()?;
        let prefix = Parser::get_prefix_parselet(&token);
        if let Some(prslt) = prefix {
            let mut left = prslt(self, token)?;
            while let Some((infix, _prec)) = Parser::get_infix_parselet(&self.peek()?) {
                token = self.next()?;
                if prec > _prec {
                    break
                } else {
                    left = infix(self, token, left)?
                }
            }
            Ok(left)
        } else {
            Err(ParserError::NoParseletFound("prefix".into(), token))
        }
    }

    fn next(&mut self) -> ParserResult<Token> {
        if self.index < self.tokens.len() {
            self.index += 1;
            Ok(self.tokens[self.index - 1].clone())
        } else {
            Err(ParserError::NoMoreTokens)
        }
    }

    fn peek(&mut self) -> ParserResult<Token> {
        if self.index < self.tokens.len() {
            Ok(self.tokens[self.index].clone())
        } else {
            Err(ParserError::NoMoreTokens)
        }
    }
}
