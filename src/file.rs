use crate::parser::Facts;
use crate::solver::Solver;

use std::fs::File;
use std::io::{Error, BufReader, prelude::*}; // ErrorKind, 

pub fn output_result(fname: &str, facts: &Facts) -> Result<File, Error> {
	let mut f = File::create(fname)?;
	let mut fcontents = String::new();
	for (index, fact) in facts.fact_arr.iter().enumerate() {
		if fact.queried == true {
			let c = (index as u8 + b'A') as char;
			fcontents.push(c);
			match fact.state {
				true	=> fcontents.push_str(" = TRUE\n"),
				false	=> fcontents.push_str(" = FALSE\n"),
			}
		}
	}
	f.write_all(fcontents.as_bytes())?;
	println!("The output result has been printed in the following file : {}", fname);
	Ok(f)
}

pub fn	parser(file: &File) -> Result<Solver, Error> {
	let reader = BufReader::new(file);
	let mut solver = Solver::new();
	for line in reader.lines() {
		let line = line.unwrap();
		match line.chars().next() {
			Some('=')	=> solver.facts.set_initial_facts(&line),
			// Some('A')	=> solver.facts.set_rule(&line, &mut solver),
			Some('?')	=> solver.facts.set_queries(&line),
			// Some(c)		=> return Err(Error::new(ErrorKind::InvalidData, format!("Input file has a format error (char {})", c))),
			_			=> continue,
		};
	}
	println!("[file::parser]FILE PARSED\n");
//	facts.get('V');
//	facts.set('V', "query", true);
	Ok(solver)
}
