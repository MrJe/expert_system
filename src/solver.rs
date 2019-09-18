mod rule;

use rule::{Rule, token::Operand};
use crate::parser::Facts;

pub struct	Solver<'a> {
	pub rules:	Vec<Rule<'a>>,
}

impl<'a> Solver<'a> {
	pub fn	new() -> Solver<'a> {
		Solver {
			rules: Vec::new(),
		}
	}

	pub fn set_rule(&mut self, facts: &'a Facts, line: &str) {
		let mut rule = Rule::new();
		for c in line.chars() {
			if c.is_whitespace() {
				continue;
			} else if c.is_uppercase() {
				rule.push(None, Some(facts.get(c)));
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
		self.rules.push(rule);
	}

	pub fn	print(&self) {
		println!("PRINTING RULES");
		for rule in &self.rules {
			rule.print();
		}
	}
}
