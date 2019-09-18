use crate::parser::Facts;
use crate::solver::Solver;

use std::fs::File;
use std::io::{Error, ErrorKind, BufReader, prelude::*};

pub fn output_result(fname: &str, facts: &Facts) -> Result<File, Error> {
	let mut f = File::create(fname)?;
	let mut fcontents = String::new();
	for (index, fact) in facts.fact_arr.iter().enumerate() {
		if fact.queried.get() == true {
			let c = (index as u8 + b'A') as char;
			fcontents.push(c);
			match fact.state.get() {
				true	=> fcontents.push_str(" = TRUE\n"),
				false	=> fcontents.push_str(" = FALSE\n"),
			}
		}
	}
	f.write_all(fcontents.as_bytes())?;
	println!("The output result has been printed in the following file : {}", fname);
	Ok(f)
}

pub fn	parser(file: &File) -> Result<Facts, Error> {
	let reader = BufReader::new(file);
	let facts = Facts::new();
	let mut solver = Solver::new();
	for line in reader.lines() {
		let line = line.unwrap();
		match line.trim().chars().next() {
			Some('A' ..= 'Z') | Some('(')	=> solver.set_rule(&facts, &line)?,
			Some('=')						=> facts.set_initial_facts(&line),
			Some('?')						=> facts.set_queries(&line),
			Some('#') | None				=> continue,
			_								=> return Err(Error::new(ErrorKind::InvalidData, format!("Input file has a format error (line {})", &line))),
		};
	}
	println!("[file::parser]FILE PARSED\n");
	// TEST THAT IT ALL WORKS -> OK
	// solver.rules[0].lhs[0].set_state(true);
	solver.print();
	Ok(facts)
}
