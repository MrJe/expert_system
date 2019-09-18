use std::env;
use std::fs::File;

fn expert_system(file: File) {
	println!("expert_system() launched !");
	let facts = lib::file::parser(&file).unwrap();
	facts.print('A');
	facts.print('B');
	facts.print('C');
	facts.print('V');
	facts.print('X');
	// solver.rules_printer();
	// let _res = lib::file::output_result("RESULT.txt", &facts);
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		println!("usage: ./expert_system input_file");
	}
	else {
		let filename = &args[1];
		match File::open(filename) {
			Ok(file)	=> expert_system(file),
			Err(error)	=> println!("open: {}: {}", filename, error),
		};
	}
}
