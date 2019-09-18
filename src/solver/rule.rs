pub mod token;

use token::{Token, Operand};
use crate::parser::Fact;

// type Rule<'a> = Vec<Token<'a>>
pub struct	Rule<'a> {
	pub tokens: Vec<Token<'a>>,
	// pub lhr: Vec<Token>,
	// pub rhs: Vec<Token>, // &mut Fact -> Vec bc of XOR in ccl
	// pub is_biconditional: bool,
}


impl<'a> Rule<'a> {
	pub fn	new() -> Rule<'a> {
		Rule {
			tokens: Vec::new(),
			// lhs: Vec::new(),
			// rhs: Vec::new(),
			// is_biconditional: false,
		}
	}

	pub fn	push(&mut self, operand: Option<Operand>, fact: Option<&'a Fact>) {
		self.tokens.push(Token::new(operand, fact));
	}

	pub fn	print(&self) {
		println!("Rule:");
		for token in &self.tokens {
			println!("\tToken: operand {:?}, fact: {:?}", token.operand, token.fact);
		}
	}
}
