use crate::ruler::rule::{Side, token::Token};
use std::io::{Error, ErrorKind};

pub fn impliance(side: &mut Side, &c: &char) -> Result<(), Error> {
	if *side == Side::Rhs {
		return Err(Error::new(
			ErrorKind::InvalidData,
			"Rules: <, =, or > oversupplied",
		));
	}
	let side_cpy: Side = *side;
	match c {
		'=' => *side = Side::Pending,
		'<' => *side = Side::Bidirectional,
		'>' => *side = Side::Rhs,
		_ => {
			return Err(Error::new(
				ErrorKind::InvalidData,
				"Rules: impliance wrong format",
			))
		}
	};
	if side_cpy == *side {
		return Err(Error::new(
			ErrorKind::InvalidData,
			"Rules: impliance wrong format",
		));
	}
	Ok(())
}

pub fn rule_composition(tokens: &Vec<Token>, line: &str) -> Result<(), Error> {
	let mut last = &Token::new(None, None);
	for token in tokens {
		if !last.is_empty() {
			if token.is_fact() && last.is_fact() {
				return Err(Error::new(
					ErrorKind::InvalidData,
					format!("Rules: contiguous facts (at {})", line),
				));
			}
			if token.is_operand() && !token.is_cumulable() && last.is_operand() {
				return Err(Error::new(
					ErrorKind::InvalidData,
					format!("Rules: contiguous operands (at {})", line),
				));
			}
		}
		if !token.is_cumulable() {
			last = token;
		}
	}
	Ok(())
}
