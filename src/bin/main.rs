fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("usage: ./expert_system input_file");
    } else {
        lib::expert_system::run(&args[1]);
    }
}
