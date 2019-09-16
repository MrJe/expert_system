// when there are priorities, just call smth like `rule_definer() -> bool` recursively 
use crate::parser::Fact;

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

	// pub fn	clean(&mut self) {
	// 	self = None;
	// }
}

pub struct	Solver<'a> {
	pub rule_v: Vec<Rule<'a>>,
	pub cur_rule: Option<&'a mut Rule<'a>>,
}

impl<'a> Solver<'a> {
	pub fn	new() -> Solver<'a> {
		Solver {
			rule_v: Vec::new(),
			cur_rule: None,
		}
	}

	pub fn	iterate_solved(&self) {}
	pub fn	rule_tokenizer(&self) {}
	pub fn	rule_solver(&self) {}

	// pub fn	get_cur(&self) -> Option<&'a mut Rule> {
	// 	self.cur_rule
	// }

	pub fn	set_cur(&mut self, ref_rule: &'a mut Rule<'a>) {
		self.cur_rule = Some(ref_rule);
	}
}
