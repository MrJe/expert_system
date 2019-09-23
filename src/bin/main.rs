// the main entry point for packages producing executables
use std::env;
use std::fs;
use lib::expert_system::expert_system;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: ./expert_system input_file");
    }
    else {
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(sfile)   => expert_system(&sfile),
            Err(error)  => println!("open: {}: {}", filename, error),
        };
    }
}
