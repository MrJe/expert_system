// when there are priorities, just call smth like `rule_definer() -> bool` recursively 
use crate::parser::{Fact, Facts};

#[derive(Copy, Clone, Debug)]
pub enum	Operand {
	And,
	Or,
	Xor,
	Not,
}

pub struct	Token<'a> {
	pub operand: Option<Operand>,
	pub fact: Option<&'a Fact>,
}

impl<'a> Token<'a> {
	pub fn	new(operand: Option<Operand>, fact: Option<&'a Fact>) -> Token<'a> {
		Token {
			operand,
			fact,
		}
	}
}

pub struct	Rule<'a> {
	pub token_v: Vec<Token<'a>>,
	// pub lhr: Vec<Token>,
	// pub rhs: Vec<Token>, // &mut Fact -> Vec bc of XOR in ccl
	// pub is_biconditional: bool,
}

impl<'a> Rule<'a> {
	pub fn	new() -> Rule<'a> {
		Rule {
			token_v: Vec::new(),
			// lhs: Vec::new(),
			// rhs: Vec::new(),
			// is_biconditional: false,
		}
	}

	pub fn	push(&mut self, operand: Option<Operand>, fact: Option<&'a Fact>) {
		self.token_v.push(Token::new(operand, fact));
	}

	pub fn	print(&self) {
		println!("Rule:");
		for token in &self.token_v {
			println!("\tOperand : {:?}, fact: {:?}", token.operand, token.fact);
		}
	}
}

pub struct	Solver<'a> {
	pub rule_v:		Vec<Rule<'a>>,
	pub cur_rule:	Option<&'a mut Rule<'a>>,
	pub facts:		Facts
}

impl<'a> Solver<'a> {
	pub fn	new() -> Solver<'a> {
		Solver {
			rule_v: Vec::new(),
			cur_rule: None,
			facts: Facts::new(),
		}
	}

	pub fn	iterate_solved(&self) {}
	pub fn	rule_tokenizer(&self) {}
	pub fn	rule_solver(&self) {}

	pub fn	rules_printer(&self) {
		println!("PRINTING RULES");
		for rule in &self.rule_v {
			rule.print();
		}
	}

	pub fn	set_cur(&mut self, ref_rule: &'a mut Rule<'a>) {
		self.cur_rule = Some(ref_rule);
	}
}
