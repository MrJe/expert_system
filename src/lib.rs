mod file;
mod rule;
mod query;
mod fact;
#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

pub fn modules_checker() -> Result<bool, Error> { // just to try, don't use
	let input_file = file::open("example_input.txt")?;
	file::read(false)?;
	file::parser()?;
	file::print(&input_file)?;
	let output_file = file::create("output_test.txt")?;
	file::write(&output_file, "Here is our first file created in Rust !")?;
	Ok(true)
}


pub fn get_file(fname: &str) -> Result<(File, String), Error> {
	let mut f = File::open(fname)?;
	let mut contents = String::new();
	f.read_to_string(&mut contents)?;
	Ok((f, contents))
}

pub fn output_to_file(fname: &str) -> Result<bool, Error> {
	let mut f = file::create(fname)?;
	f.write_all(b"Here is our first file created in Rust !")?;
	println!("The output result has been printed in the following file : {}", fname);
	Ok(true)
}
