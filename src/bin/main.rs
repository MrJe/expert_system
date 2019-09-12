fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() == 2 {
		println!("{}", &args[1]);
		let parsed_content;
		match lib::file::get_contents(&args[1]) {
			Ok((_, contents))	=> parsed_content = lib::file::parser(&contents).unwrap(),
			Err(err)			=> return println!("{}", err)
		}
		parsed_content.print('V');
		lib::file::output_result("RESULT.txt");
	}
	else {
		println!("Usage: only 2 parameters !");
	}
}
