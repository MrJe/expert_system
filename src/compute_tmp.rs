pub type Statement = bool;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum	Operand {
	And,
	Or,
	Xor,
	Not,
	Opening,
	Closing,
}

pub struct	Token {
	pub pending_op: Option<Operand>,
	pub fact: Option<bool>,
}

impl Token {
	pub fn	new(pending_op: Option<Operand>, fact: Option<bool>) -> Token {
		Token { pending_op, fact }
	}

	pub fn	is_fact(&self) -> bool {
		self.fact.is_some()
	}

	pub fn	is_pending_op(&self) -> bool {
		self.pending_op.is_some()
	}

	pub fn	set(&mut self, new_state: bool) {
		self.fact = Some(new_state);
	}
}


pub struct Computer {
	pending_op: Option<Operand>,
	is_not: bool,
	res: Option<Statement>,
}

impl	Computer {
	pub fn new() -> Computer {
		Computer { pending_op: None, is_not: false, res: None }
	}

	// pub fn compute(&mut self, rules: &Vec<Token>) {
	// 	for rule in rules {
	// 		if rule.is_pending_op() {
	// 			self.store_op(rule.pending_op);
	// 		} else if self.res.is_some() {
	// 			self.compute_each(rule.fact.unwrap());
	// 		} else {
	// 			self.init(&rule);
	// 		}
	// 	}
	// }

	pub fn compute(&mut self, rules: &Vec<Token>) -> bool {
		for token in rules {
			if token.is_pending_op() {
				if token.pending_op.unwrap() == Operand::Opening {
					self.res = Some(self.compute(rules)); // rules from token + 1
				}
				if token.pending_op.unwrap() == Operand::Closing {
					return self.res.unwrap();
				}
				self.store_op(token.pending_op);
			} else if self.res.is_some() {
				self.compute_each(token.fact.unwrap());
			} else {
				self.init(&token);
			}
		};
		self.res.unwrap_or(false)
	}

	pub fn print(&self) {
		println!("Sum of tokens is = {:?}", self.res);
	}

	fn init(&mut self, token: &Token) {
		if token.fact.is_some() {
			self.res = Some(token.fact.unwrap() ^ self.is_not);
		}
		else {
			panic!("FIRST ELEM SHOULD BE A FACT");
		}
	}

	fn store_op(&mut self, pending_op: Option<Operand>) {
		if pending_op.unwrap() == Operand::Not {
			self.is_not = true;
		} else {
			self.pending_op = pending_op;
		}
	}

	fn compute_each(&mut self, rhs: Statement) {
		let mut lhs = self.res.unwrap();
		match self.pending_op.unwrap() {
			Operand::And	=> lhs &= rhs ^ self.is_not,
			Operand::Or		=> lhs |= rhs ^ self.is_not,
			Operand::Xor	=> lhs ^= rhs ^ self.is_not,
			_				=> (),
		}
		println!("RESL : {} {:?} {} = {}", self.res.unwrap(), self.pending_op, rhs ^ self.is_not, lhs);
		self.res = Some(lhs);
		self.prepare_next();
	}

	fn	prepare_next(&mut self) {
		self.pending_op = None;
		self.is_not = false;
	}
}

fn main() {
	let rules = vec![
		Token::new(Some(Operand::Not), None),
		Token::new(None, Some(false)),
		Token::new(Some(Operand::And), None),
		Token::new(Some(Operand::Not), None),
		Token::new(None, Some(true)),
		Token::new(Some(Operand::And), None),
		Token::new(None, Some(true)),
		Token::new(Some(Operand::Or), None),
		Token::new(None, Some(true)),
		Token::new(Some(Operand::Xor), None),
		Token::new(None, Some(true)),
	];
	let mut sum = Computer::new();
	sum.compute(&rules);
	sum.print();
}

// A & B & C | D ^ E => X
// =BCDE
// ?X
