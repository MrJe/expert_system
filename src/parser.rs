use std::cell::Cell;

#[derive(Clone, Debug)]
pub struct Fact{
	pub state: Cell<bool>,
	pub queried: Cell<bool>,
}

impl	Fact {
	pub fn	new() -> Fact {
		Fact {
			state: Cell::new(false),
			queried: Cell::new(false),
		}
	}
}

pub struct Facts {
	pub fact_arr: [Fact; 26],
	pub is_stable: bool,
}

impl Facts {
	pub fn new() -> Facts {
		let arr = [Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new(), Fact::new()];

		Facts {
			fact_arr: arr,
			is_stable: false,
		}
	}

	pub fn get(&self, letter: char) -> &Fact {
		&self.fact_arr[self.get_index(letter)]
	}

	pub fn set_initial_facts(&self, line: &str) {
		for c in line.chars() {
			if c == '#' {
				break;
			}
			if c.is_uppercase() {
				self.fact_arr[self.get_index(c)].state.set(true);
			}
		}
	}

	pub fn set_queries(&self, line: &str) {
		for c in line.chars() {
			if c == '#' {
				break;
			}
			if c.is_uppercase() {
				self.fact_arr[self.get_index(c)].queried.set(true);
			}
		}
	}

	pub fn print(&self, letter: char) {
		let index = self.get_index(letter);
		let res = &self.fact_arr[index];
		println!("print parser element : {} (index {}) =>\n{:?}", letter, index, res);
	}

	// fn set_value(&mut self, attr: &str, index: usize, value: bool) {
	// 	let fact = &self.fact_arr[index];
	// 	match attr {
	// 		"state"			=> fact.state.set(value),
	// 		"queried"		=> fact.queried.set(value),
	// 		_				=> panic!("[{}] Attribute does not exist", attr),
	// 	}
	// }

	fn get_index(&self, letter: char) -> usize {
		if letter.is_uppercase() {
			letter as usize - 65
		}
		else {
			panic!("[{}] Letter does not match any valid index", letter);
		}
	}
}
