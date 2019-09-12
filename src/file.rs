use crate::entities::Entities as Entities;

use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

pub fn get_contents(fname: &str) -> Result<(File, String), Error> {
	let mut f = File::open(fname)?;
	let mut contents = String::new();
	f.read_to_string(&mut contents)?;
	Ok((f, contents))
}

pub fn output_result(fname: &str) {
	let mut f = File::create(fname)?;
	f.write_all(b"Here is our first file created in Rust !")?;
	println!("The output result has been printed in the following file : {}", fname);
}

pub fn	parser(fcontents: &str) -> Result<Entities, Error> { // Result<struct, Error>
	println!("[file::parser]File contents: {}", fcontents);
	let mut entities = Entities::new();
	entities.get('V');
	entities.set('V', "query", true);
	entities.set('V', "is_initial", true);
	println!("[file::parser]FILE PARSED\n");
	Ok(entities)
}
