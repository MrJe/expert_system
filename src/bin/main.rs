use lib::expert_system::run_ep;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: ./expert_system input_file");
    } else {
        run_ep(&args[1]);
    }
}
