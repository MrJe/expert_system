mod rule;

use std::io::{Error, ErrorKind};
use rule::{Rule, Side, token::Operand};
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

	fn		impliance_checker(side: &mut Side, &c: &char) -> Result<(), Error> {
		let side_cpy: Side = *side;
		match c {
			'='		=> *side = Side::Pending,
			'<'		=> *side = Side::Bidirectional,
			'>'		=> *side = Side::Rhs,
			_		=> return Err(Error::new(ErrorKind::InvalidData, "Rules: impliance wrong format"))
		};
		if side_cpy == *side {
			return Err(Error::new(ErrorKind::InvalidData, "Rules: impliance wrong format"));
		}
		Ok(())
	}

	pub fn set_rule(&mut self, facts: &'a Facts, line: &str) -> Result<(), Error> {
		let mut side = Side::Lhs;
		let mut rule = Rule::new();
		
		for c in line.chars() {
			if side == Side::Pending || side == Side::Bidirectional {
				if side == Side::Bidirectional {
					rule.is_equivalent = true;
				}
				Solver::impliance_checker(&mut side, &c)?;
				continue;
			}
			if c.is_whitespace() {
				continue;
			} else if c.is_uppercase() {
				rule.push(&side, None, Some(facts.get(c)));
			}
			else {
				match c {
					'!'	=> rule.push(&side, Some(Operand::Not), None),
					'|'	=> rule.push(&side, Some(Operand::Or), None),
					'^'	=> rule.push(&side, Some(Operand::Xor), None),
					'+'	=> rule.push(&side, Some(Operand::And), None),
					'#'	=> break,
					'<' | '=' => Solver::impliance_checker(&mut side, &c)?,
					_	=> continue,
				};
			}
		}
		if side != Side::Rhs {
			return Err(Error::new(ErrorKind::InvalidData, "Rules: no impliance"));
		}
		self.rules.push(rule);
		Ok(())
	}

	pub fn	print(&self) {
		println!("PRINTING RULES");
		for rule in &self.rules {
			rule.print();
		}
	}
}
