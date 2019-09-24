const OUTPUT_FILE: &str = "RESULT.txt";

use lib::parser::Facts;
use lib::solver::Solver;

use std::fs::File;
use std::io::{prelude::*, BufReader, Error, ErrorKind};

fn print_solved_to_file(fname: &str, facts: Facts) -> Result<(), Error> {
    let mut f = File::create(fname)?;
    let mut fcontents = String::new();
    for (index, fact) in facts.fact_arr.iter().enumerate() {
        if fact.queried.get() == true {
            let c = (index as u8 + b'A') as char;
            fcontents.push(c);
            match fact.state.get() {
                true => fcontents.push_str(" = TRUE\n"),
                false => fcontents.push_str(" = FALSE\n"),
            }
        }
    }
    f.write_all(fcontents.as_bytes())?;
    println!(
        "The output result has been printed in the following file : {}",
        fname
    );
    Ok(())
}

fn parser<'a>(file: &File, facts: &'a Facts) -> Result<Solver<'a>, Error> {
    let reader = BufReader::new(file);
    let mut solver = Solver::new();
    for line in reader.lines() {
        let line = line.unwrap();
        match line.trim().chars().next() {
            Some('A'..='Z') | Some('(') | Some('!') => solver.set_rule(&facts, &line)?,
            Some('=') => facts.set_initial_facts(&line)?,
            Some('?') => facts.set_queries(&line)?,
            Some('#') | None => continue,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Input file has a format error (line {})", &line),
                ))
            }
        }
    }
    Ok(solver)
}

fn solver(file: &File) -> Result<Facts, Error> {
    let facts = Facts::new();
    let mut _solver = parser(file, &facts)?;
    // solver.to_rpn(); TODO HERE
    Ok(facts)
}

fn expert_system(file: File) {
    let solved_facts: Facts = match solver(&file) {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            let fact: Facts = Facts::new();
            dbg!(e);
            fact
        }
    };
    match print_solved_to_file(OUTPUT_FILE, solved_facts) {
        Ok(()) => println!(
            "Expert system worked as expected, open {} to see the results!",
            OUTPUT_FILE
        ),
        Err(error) => println!(
            "Oops, something went wrong, shutting program down.\n Error: {}",
            error
        ),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("usage: ./expert_system input_file");
    } else {
        let filename = &args[1];
        match File::open(filename) {
            Ok(file) => expert_system(file),
            Err(error) => println!("open: {}: {}", filename, error),
        };
    }
}
