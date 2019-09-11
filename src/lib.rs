mod file;
mod rule;
mod query;
mod fact;
#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

pub fn get_file_contents(fname: &str) -> Result<(File, String), Error> {
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
