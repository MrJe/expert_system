// fn main() {
// 	println!("I'm using the library: {:?}", lib::modules_checker().unwrap());
// }

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() == 2 {
		println!("{}", &args[1]);
		match lib::get_file_contents(&args[1]) {
			Ok((_, contents))	=> println!("FILE CONTENTS : {}", contents),
			Err(err)			=> return println!("{}", err)
		}
		let _output_file = lib::output_to_file("RESULT.txt");
	}
	else {
		println!("Usage: only 2 parameters !");
	}
}
