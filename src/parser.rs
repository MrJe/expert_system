use crate::solver::{Operand, Rule, Solver};

#[derive(Copy, Clone, Debug)]
pub struct Fact {
	pub state: bool,
	pub queried: bool,
	pub is_initial: bool
}

impl Fact {
	pub fn	new() -> Fact {
		Fact {
			state: false,
			queried: false,
			is_initial: false,
		}
	}
}

pub struct Facts {
	pub fact_arr: [Fact; 26],
	pub is_stable: bool,
}

impl Facts {
	pub fn new() -> Facts {
		Facts {
			fact_arr: [ Fact::new() ; 26 ],
			is_stable: false
		}
	}

	pub fn get(&self, letter: char) -> &Fact {
		&(self.fact_arr[self.get_index(letter)])
	}

	pub fn set(&mut self, letter: char, attr: &str, value: bool) {
		let index = self.get_index(letter);
		*self.set_value(attr, index) = value;
	}

	pub fn set_rule<'a>(&'a mut self, line: &str, solver: &'a mut Solver<'a>) {
		let mut rule = Rule::new();
		for c in line.chars() {
			if c.is_whitespace() {
				continue;
			} else if c.is_uppercase() {
				rule.push(None, Some(self.get(c)));
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
		solver.rule_v.push(rule);
	}

	pub fn set_initial_facts(&mut self, line: &str) {
		for c in line.chars() {
			if c == '#' {
				break;
			}
			if c.is_uppercase() {
				self.fact_arr[self.get_index(c)].is_initial = true;
				self.fact_arr[self.get_index(c)].state = true;
			}
		}
	}

	pub fn set_queries(&mut self, line: &str) {
		for c in line.chars() {
			if c == '#' {
				break;
			}
			if c.is_uppercase() {
				self.fact_arr[self.get_index(c)].queried = true;
			}
		}
	}

	pub fn print(&self, letter: char) {
		let index = self.get_index(letter);
		let res = self.fact_arr[index];
		println!("print parser element : {} (index {}) =>\n{:?}", letter, index, res);
	}

	fn set_value(&mut self, attr: &str, index: usize) -> &mut bool {
		match attr {
			"state"			=> return &mut(self.fact_arr[index].state),
			"queried"		=> return &mut(self.fact_arr[index].queried),
			"is_initial"	=> return &mut(self.fact_arr[index].is_initial),
			_				=> panic!("[{}] Attribute does not exist", attr),
		}
	}

	fn get_index(&self, letter: char) -> usize {
		if letter.is_uppercase() {
			letter as usize - 65
		}
		else {
			panic!("[{}] Letter does not match any valid index", letter);
		}
	}
}
