use std::env;
use std::fs::File;

fn expert_system(file: File) {
	println!("expert_system() launched !");
	// let mut facts = lib::parser::Facts::new();
	let solver = lib::file::parser(&file).unwrap();
	solver.facts.print('A');
	solver.facts.print('B');
	solver.facts.print('C');
	solver.facts.print('V');
	solver.facts.print('X');
	solver.rules_printer();
	// let _res = lib::file::output_result("RESULT.txt", &(solver.facts));

}

fn main() {
	let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: ./expert_system input_file");
    }
    else {
        let filename = &args[1];
        match File::open(filename) {
            Ok(file)    => expert_system(file),
            Err(error)  => println!("open: {}: {}", filename, error),
        };
    }
}
