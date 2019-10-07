use lib::expert_system;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: ./expert_system input_file");
    } else {
        expert_system::run(&args[1]);
    }
}
