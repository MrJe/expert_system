use std::env;
use std::fs;

/*
** enum Operand { And, Or, Xor, Not, }
** struct Token { operand: Option<Operand>, fact: Option<Fact>, }
** struct Rule { token_v: Vec<Token> }
** struct Solver { rule_v: Vec<Rule>, facts: Facts }
** struct Fact { state: bool, queried: bool, is_initial: bool }
*/

#[derive(Copy, Clone, Debug)]
struct Fact {
    letter: char,
	state: bool,
	queried: bool,
	nb_change: i32,
}

impl Fact {
	fn	new() -> Self {
		Fact {
            letter: '*',
			state: false,
			queried: false,
			nb_change: 0,
		}
	}

    fn  print(&self) {
        println!("{}, {}, {}, {}", self.letter, self.state, self.queried, self.nb_change);
    }
}

fn init_facts(facts: &mut [Fact]) {
    for (i, f) in facts.iter_mut().enumerate() {
        f.letter = (65 + i as u8) as char;
    }
}

fn print_facts(facts: &[Fact]) {
    for f in facts {
        f.print();
    }
}

fn expert_system(file: &str) {
    let     lines = file.lines();
    let mut facts: [Fact; 26] = [Fact::new(); 26];

    init_facts(&mut facts);
    for s in lines {
        println!("line: {}", s);
    }
    print_facts(&facts);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: ./expert_system input_file");
    }
    else {
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(sfile)    => expert_system(&sfile),
            Err(error)  => println!("open: {}: {}", filename, error),
        };
    }
}
