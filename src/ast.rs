use crate::lexer::{Token, Span};
use std::collections::HashMap;

pub type ParameterList = HashMap<String, Type>;
type WithSpan<T> = (T, Span);

#[derive(Debug, Clone)]
pub enum Type {
	Any,
	Interface(String),
	Object,
	Function {
		parameters: Vec<Type>,
		return_type: Box<Type>,
	},
	Matrix {
		width: usize,
		height: usize,
	},
	String,
	Null,
	Integer,
	Float,
}

#[derive(Debug, Clone)]
pub enum Stat {
	Expression(Expr),
	Assignment(Assignment),
	Return(Return),
}

#[derive(Debug, Clone)]
pub struct Return {
	expr: Option<Expr>
}

#[derive(Debug, Clone)]
pub struct Assignment {
	target: Expr,
	value: Expr,
}

impl Assignment {
	pub fn new_stat(target: Expr, value: Expr) -> Stat {
		Stat::Assignment(Assignment {
			target, value,
		})
	}
}

#[derive(Debug, Clone)]
pub enum Expr {
	Identifier(Identifier),
	Integer(Integer),
	Float(Float),
	SubAccess(SubAccess),
	Call(Call),
	NewObject(NewObject),
	Function(Function),
	Binary(Binary),
}

impl Expr {
	pub fn is_identifier(&self) -> bool {
		unimplemented!()
	}

	pub fn as_identifier(&self) -> Option<Identifier> {
		unimplemented!()
	}

	pub fn is_binary(&self) -> bool {
		unimplemented!()
	}

	pub fn as_binary(&self) -> Option<Binary> {
		unimplemented!()
	}
}

#[derive(Debug, Clone)]
pub struct Identifier {
	ident: WithSpan<String>,
}

impl Identifier {
	pub fn new_expr(tok: WithSpan<String>) -> Expr {
		Expr::Identifier(Identifier {
			ident: tok,
		})
	}
}

#[derive(Debug, Clone)]
pub struct Binary {
	lhs: Box<Expr>,
	rhs: Box<Expr>,
	op: Token,
}

impl Binary {
	pub fn new_expr(lhs: Expr, rhs: Expr, op: Token) -> Expr {
		Expr::Binary(Binary {
			lhs: Box::new(lhs), rhs: Box::new(rhs), op,
		})
	}

	pub fn op(&self) -> Token { self.op.clone() }
}

#[derive(Debug, Clone)]
pub struct Integer {
	tok: Token,
	number: i64
}

#[derive(Debug, Clone)]
pub struct Float {
	tok: Token,
	number: f64
}

#[derive(Debug, Clone)]
pub struct Call {
	target: Box<Expr>,
	arguments: Vec<Expr>
}

#[derive(Debug, Clone)]
pub struct SubAccess {
	target: Box<Expr>,
	name: Token
}

#[derive(Debug, Clone)]
pub struct NewObject {
	basis: Box<Expr>,
	extensions: HashMap<String, Expr>,
}

#[derive(Debug, Clone)]
pub struct Function {
	parameters: ParameterList,
	return_type: Type,
	body: Vec<Stat>,
}
