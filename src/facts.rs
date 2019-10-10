use std::cell::Cell;
use std::io::{Error, ErrorKind};
use crate::options::Options;

#[derive(Clone, Debug, Default)]
pub struct Fact {
    pub state: Cell<bool>,
    pub reverse_state: Cell<bool>,
    pub determined: Cell<bool>,
    pub queried: Cell<bool>,
    pub letter: char,
}

impl Fact {
    pub fn new(letter: char) -> Self {
        Fact {
            state: Cell::new(false),
            reverse_state: Cell::new(false),
            determined: Cell::new(false),
            queried: Cell::new(false),
            letter,
        }
    }

    pub fn copy(&self) -> Self {
        Fact {
            state: Cell::new(self.state.get()),
            reverse_state: Cell::new(self.reverse_state.get()),
            determined: Cell::new(self.determined.get()),
            queried: Cell::new(self.queried.get()),
            letter: self.letter,
        }
    }

    pub fn set_solved(&self, mut expr_result: bool) {
        if self.reverse_state.get() {
            expr_result = !expr_result;
            self.reverse_state.set(false);
        }
        self.state.set(expr_result);
        if expr_result {
            self.determined.set(true);
        }
    }
}

#[derive(Default)]
pub struct Facts {
    pub fact_arr: [Fact; 26],
    pub is_stable: bool,
}

impl Facts {
    pub fn new() -> Self {
        let arr = [
            Fact::new('A'),
            Fact::new('B'),
            Fact::new('C'),
            Fact::new('D'),
            Fact::new('E'),
            Fact::new('F'),
            Fact::new('G'),
            Fact::new('H'),
            Fact::new('I'),
            Fact::new('J'),
            Fact::new('K'),
            Fact::new('L'),
            Fact::new('M'),
            Fact::new('N'),
            Fact::new('O'),
            Fact::new('P'),
            Fact::new('Q'),
            Fact::new('R'),
            Fact::new('S'),
            Fact::new('T'),
            Fact::new('U'),
            Fact::new('V'),
            Fact::new('W'),
            Fact::new('X'),
            Fact::new('Y'),
            Fact::new('Z'),
        ];

        Facts {
            fact_arr: arr,
            is_stable: false,
        }
    }

    pub fn get(&self, letter: char) -> &Fact {
        &self.fact_arr[self.get_index(letter)]
    }

    pub fn set_initial_facts(&self, line: &str, options: &Options) -> Result<(), Error> {
        let mut chars = line.chars();
        if !options.interactive || options.file  || options.comment {
            chars.next();
        }
        for c in chars {
            if c.is_whitespace() {
                continue;
            }
            if c == '>' {
                return Err(Error::new(ErrorKind::InvalidData, "Parser: empty expression found"));
            }
            match c {
                'A'..='Z' => {
                    let fact = &self.fact_arr[self.get_index(c)];
                    if fact.state.get() {
                        return Err(Error::new(ErrorKind::InvalidData, "Initial facts: doublon"));
                    }
                    fact.state.set(true);
                    fact.determined.set(true);
                }
                '#' => {
                    if options.comment && !options.file {
                        println!("{}", line);
                    }
                    break
                }
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

    pub fn set_queries(&self, line: &str, options: &Options) -> Result<(), Error> {
        let mut chars = line.chars();
        if !options.interactive || options.file  || options.comment {
            chars.next();
        }
        for c in chars {
            if c.is_whitespace() {
                continue;
            }
            match c {
                'A'..='Z' => {
                    let fact = &self.fact_arr[self.get_index(c)];
                    if fact.queried.get() {
                        return Err(Error::new(ErrorKind::InvalidData, "Queries: doublon"));
                    }
                    fact.queried.set(true);
                }
                '#' => {
                    if options.comment && !options.file {
                        println!("{}", line);
                    }
                    break
                }
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
