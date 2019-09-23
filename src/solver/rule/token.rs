use crate::parser::Fact;

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl Token<'_> {
	pub fn	new(operand: Option<Operand>, fact: Option<&Fact>) -> Token {
		Token { operand, fact }
	}

	pub fn	is_fact(&self) -> bool {
		self.fact.is_some()
	}

	pub fn	is_operand(&self) -> bool {
		self.operand.is_some()
	}

	pub fn	set_state(&self, new_state: bool) {
		match self.fact {
			Some(fact)	=> fact.state.set(new_state),
			None		=> return,
		}
	}
}
