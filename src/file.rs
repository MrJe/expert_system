use crate::parser::Facts;

use std::fs::File;
use std::io::{Error, ErrorKind, BufReader, prelude::*};

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

pub fn	parse(file: &File) -> Result<Facts, Error> {
	let reader = BufReader::new(file);
	let mut facts = Facts::new();
	for line in reader.lines() {
		let line = line.unwrap();
		match line.chars().next() {
			Some('=')	=> facts.set_initial_facts(&line),
			Some('A')	=> facts.set_rule(&line),
			Some('?')	=> facts.set_queries(&line),
			// Some(c)		=> return Err(Error::new(ErrorKind::InvalidData, format!("Input file has a format error (char {})", c))),
			_			=> continue,
		}
	}
//	facts.get('V');
//	facts.set('V', "query", true);
	println!("[file::parser]FILE PARSED\n");
	Ok(facts)
}
