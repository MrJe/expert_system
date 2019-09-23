#[derive(Copy, Clone, Debug)]
pub struct Fact {
    pub letter: char,
	pub state: bool,
	pub queried: bool,
	pub nb_change: i32,
}

impl Fact {
	pub fn	new() -> Self {
		Fact {
            letter: '*',
			state: false,
			queried: false,
			nb_change: 0,
		}
	}

    pub fn  print(&self) {
        println!("{}, {}, {}, {}", self.letter, self.state, self.queried, self.nb_change);
    }
}
