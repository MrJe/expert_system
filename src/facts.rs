use std::cell::Cell;
use std::io::{Error, ErrorKind};

#[derive(Clone, Debug)]
pub struct Fact {
    pub state: Cell<bool>,
    pub queried: Cell<bool>,
}

impl Fact {
    pub fn new() -> Fact {
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
        let arr = [
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
            Fact::new(),
        ];

        Facts {
            fact_arr: arr,
            is_stable: false,
        }
    }

    pub fn get(&self, letter: char) -> &Fact {
        &self.fact_arr[self.get_index(letter)]
    }

    pub fn set_initial_facts(&self, line: &str) -> Result<(), Error> {
        let mut chars = line.chars();
        chars.next();
        for c in chars {
            if c.is_whitespace() {
                continue;
            }
            match c {
                'A'..='Z' => {
                    let fact = &self.fact_arr[self.get_index(c)];
                    if fact.state.get() == true {
                        return Err(Error::new(ErrorKind::InvalidData, "Initial facts: doublon"));
                    }
                    fact.state.set(true);
                }
                '#' => break,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Initial facts: unexpected char",
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn set_queries(&self, line: &str) -> Result<(), Error> {
        let mut chars = line.chars();
        chars.next();
        for c in chars {
            if c.is_whitespace() {
                continue;
            }
            match c {
                'A'..='Z' => {
                    let fact = &self.fact_arr[self.get_index(c)];
                    if fact.queried.get() == true {
                        return Err(Error::new(ErrorKind::InvalidData, "Queries: doublon"));
                    }
                    fact.queried.set(true);
                }
                '#' => break,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Queries: unexpected char",
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn print(&self, letter: char) {
        let index = self.get_index(letter);
        let res = &self.fact_arr[index];
        println!(
            "print parser element : {} (index {}) =>\n{:?}",
            letter, index, res
        );
    }

    fn get_index(&self, letter: char) -> usize {
        if letter.is_uppercase() {
            letter as usize - 65
        } else {
            panic!("[{}] Letter does not match any valid index", letter);
        }
    }
}
