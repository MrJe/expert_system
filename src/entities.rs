#[derive(Copy, Clone, Debug)]
pub struct Fact {
	state: bool,
	query: bool,
	is_initial: bool
}

pub struct Entities {
	facts: [Fact; 26],
	// rules: ,
	is_stable: bool,
}

impl Entities {
	pub fn new() -> Entities {
		let fact = Fact {
			state: false,
			query: false,
			is_initial: false,
		};

		Entities {
			facts: [ fact ; 26 ],
			is_stable: false
		}
	}

	pub fn get(&self, letter: char) -> &Fact {
		&self.facts[self.get_index(letter)]
	}

	pub fn set(&mut self, letter: char, attr: &str, value: bool) {
		let index = self.get_index(letter);
		*self.set_value(attr, index) = value;
	}

	pub fn print(&self, letter: char) {
		let index = self.get_index(letter);
		let res = self.facts[index];
		println!("print parser element : {} (index {}) =>\n{:?}", letter, index, res);
	}

	fn set_value(&mut self, attr: &str, index: usize) -> &mut bool {
		match attr {
			"state"			=> return &mut(self.facts[index].state),
			"query"			=> return &mut(self.facts[index].query),
			"is_initial"	=> return &mut(self.facts[index].is_initial),
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
