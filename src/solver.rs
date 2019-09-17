// when there are priorities, just call smth like `rule_definer() -> bool` recursively 
use crate::parser::{Fact, Facts};

#[derive(Copy, Clone, Debug)]
pub enum	Operand {
	And,
	Or,
	Xor,
	Not,
}

pub struct	Token {
	pub operand: Option<Operand>,
	pub fact: Option<Fact>,
}

impl Token {
	pub fn	new(operand: Option<Operand>, fact: Option<Fact>) -> Token {
		Token {
			operand,
			fact,
		}
	}
}

pub struct	Rule {
	pub token_v: Vec<Token>,
	// pub lhr: Vec<Token>,
	// pub rhs: Vec<Token>, // &mut Fact -> Vec bc of XOR in ccl
	// pub is_biconditional: bool,
}

impl Rule {
	pub fn	new() -> Rule {
		Rule {
			token_v: Vec::new(),
			// lhs: Vec::new(),
			// rhs: Vec::new(),
			// is_biconditional: false,
		}
	}

	pub fn	push(&mut self, operand: Option<Operand>, fact: Option<Fact>) {
		self.token_v.push(Token::new(operand, fact));
	}

	pub fn	print(&self) {
		println!("Rule:");
		for token in &self.token_v {
			println!("\tOperand : {:?}, fact: {:?}", token.operand, token.fact);
		}
	}
}

pub struct	Solver {
	pub rule_v:		Vec<Rule>,
	pub facts:		Facts
}

impl Solver {
	pub fn	new() -> Solver {
		Solver {
			rule_v: Vec::new(),
			facts: Facts::new(),
		}
	}

	pub fn	iterate_solved(&self) {}
	pub fn	rule_tokenizer(&self) {}
	pub fn	rule_solver(&self) {}

	pub fn set_rule(&mut self, line: &str) {
		let mut rule = Rule::new();
		for c in line.chars() {
			if c.is_whitespace() {
				continue;
			} else if c.is_uppercase() {
				rule.push(None, Some(self.facts.get(c)));
			}
			else {
				match c {
					'!'					=> rule.push(Some(Operand::Not), None),
					'|'					=> rule.push(Some(Operand::Or), None),
					'^'					=> rule.push(Some(Operand::Xor), None),
					'+'					=> rule.push(Some(Operand::And), None),
					'#'					=> break,
					_					=> continue,
				}
			}
		}
		self.rule_v.push(rule);
	}

	pub fn	rules_printer(&self) {
		println!("PRINTING RULES");
		for rule in &self.rule_v {
			rule.print();
		}
	}
}
