use super::*;

use std::fs::File;
use std::io::Error;
use std::io::BufReader;
use std::io::prelude::*;

pub fn	open(fname: &str) -> Result<File, Error> {
	println!("open file\n");
	let f = File::open(fname)?;
	Ok(f)
}
pub fn	read(mybool: bool) -> Result<bool, Error> {
	println!("read file");
	if mybool {
		error("Could not read the file");
	}
	Ok(true)
}
pub fn	parser() -> Result<bool, Error> {
	println!("parse file");
	rule::print();
	fact::print();
	query::print();
	println!("file parsed\n");
	Ok(true)
}

pub fn	print(file: &File) -> Result<bool, Error> {
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();
	buf_reader.read_to_string(&mut contents)?;
	println!("{}", contents);
	Ok(true)
}

pub fn	create(fname: &str) -> Result<File, Error> {
	let f = File::create(fname)?;
	Ok(f)
}

pub fn	write(mut file: &File, contents: &str) -> Result<bool, Error> {
	file.write_all(contents.as_bytes())?;
	Ok(true)
}

fn		error(err: &str) {
	panic!("A BAD error occured : {:?}", err)
}
