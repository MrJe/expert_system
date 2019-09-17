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
	operand: Option<Operand>,
	fact: Option<&'a Fact>,
}

impl Token<'_> {
	fn	new(operand: Option<Operand>, fact: Option<&Fact>) -> Token {
		Token { operand, fact }
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
			println!("\tToken: operand {:?}, fact: {:?}", token.operand, token.fact);
		}
	}
}

pub struct	Solver<'a> {
	pub rule_v:		Vec<Rule<'a>>,
	pub facts:		Facts
}

impl<'a> Solver<'a> {
	pub fn	new() -> Solver<'a> {
		Solver {
			rule_v: Vec::new(),
			facts: Facts::new(),
		}
	}

	pub fn set_rule(&mut self, line: &str) {
		let mut rule = Rule::new();
		for c in line.chars() {
			if c.is_whitespace() {
				continue;
			} else if c.is_uppercase() {
				rule.push(None, Some(self.facts.get(c))); // ISSUE HERE
			}
			else {
				match c {
					'!'	=> rule.push(Some(Operand::Not), None),
					'|'	=> rule.push(Some(Operand::Or), None),
					'^'	=> rule.push(Some(Operand::Xor), None),
					'+'	=> rule.push(Some(Operand::And), None),
					'#'	=> break,
					_	=> continue,
				}
			}
		}
		self.rule_v.push(rule);
	}

	pub fn	print(&self) {
		println!("PRINTING RULES");
		for rule in &self.rule_v {
			rule.print();
		}
	}
}
