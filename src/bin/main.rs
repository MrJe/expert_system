const OUTPUT_FILE: &str = "RESULT.txt";

use lib::facts::Facts;
use lib::ruler::Ruler;

use std::fs::File;
use std::io::{prelude::*, BufReader, Error, ErrorKind};


fn print_results(facts: Facts) {
    println!("Everything worked as expected, here are the results:");
    for (index, fact) in facts.fact_arr.iter().enumerate() {
        if fact.queried.get() == true {
            let c = (index as u8 + b'A') as char;
            print!("{}", c);
            match fact.state.get() {
                true => println!(" = TRUE"),
                false => println!(" = FALSE"),
            }
        }
    }
}

fn print_solved_to_file(fname: &str, facts: &Facts) -> Result<(), Error> {
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
    // println!(
    //     "The output result has been printed in the following file : {}",
    //     fname
    // );
    Ok(())
}

fn parser<'a>(file: &File, facts: &'a Facts) -> Result<Ruler<'a>, Error> {
    let reader = BufReader::new(file);
    let mut ruler = Ruler::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            match line.trim().chars().next() {
                Some('A'..='Z') | Some('(') | Some('!') => ruler.set_rule(&facts, &line)?,
                Some('=') => facts.set_initial_facts(&line)?,
                Some('?') => facts.set_queries(&line)?,
                Some('#') | None => continue,
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Input file: unexpected char (line: {})", line),
                    ))
                }
            }
        }
    }
    Ok(ruler)
}

fn solver(file: &File) -> Result<Facts, Error> {
    let facts = Facts::new();
    let mut ruler = parser(file, &facts)?;
    ruler.to_reverse_polish_notation()?;
    ruler.print();
    ruler.solve()?;
    Ok(facts)
}

fn expert_system(file: File) -> Result<Facts, Error> {
    let solved_facts: Facts = solver(&file)?;
    print_solved_to_file(OUTPUT_FILE, &solved_facts)?;
    Ok(solved_facts)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("usage: ./expert_system input_file");
    } else {
        let filename = &args[1];
        match File::open(filename) {
            Ok(file) => match expert_system(file) {
                Ok(facts) => print_results(facts),
                Err(error) => println!(
                    "Oops, something went wrong, shutting program down.\nError:\n{:#?}",
                    error
                ),
            },
            Err(error) => println!("open: {}: {:#?}", filename, error),
        };
    }
}
