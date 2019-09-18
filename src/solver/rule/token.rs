use crate::parser::Fact;

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

impl Token<'_> {
	pub fn	new(operand: Option<Operand>, fact: Option<&Fact>) -> Token {
		Token { operand, fact }
	}
}
