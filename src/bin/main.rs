fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() == 2 {
		println!("{}", &args[1]);
		let f = std::fs::File::open(&args[1]).unwrap();
		// let mut facts = lib::parser::Facts::new();
		let solver = lib::file::parser(&f).unwrap();
		solver.facts.print('A');
		solver.facts.print('B');
		solver.facts.print('C');
		solver.facts.print('V');
		solver.facts.print('X');
		solver.rules_printer();
		let _res = lib::file::output_result("RESULT.txt", &(solver.facts));
	}
	else {
		println!("Usage: only 2 parameters !");
	}
}
