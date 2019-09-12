fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() == 2 {
		println!("{}", &args[1]);
		let f = std::fs::File::open(&args[1]).unwrap();
		let facts = lib::file::parse(&f).unwrap();
		facts.print('A');
		facts.print('B');
		facts.print('C');
		facts.print('V');
		facts.print('X');
		lib::file::output_result("RESULT.txt", &facts);
	}
	else {
		println!("Usage: only 2 parameters !");
	}
}
