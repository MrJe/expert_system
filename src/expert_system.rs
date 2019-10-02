const OUTPUT_FILE: &str = "RESULT.txt";

use crate::checker;
use crate::facts::Facts;
use crate::rules::Rules;
use crate::print::{print_solved_to_file, print_results};

use std::fs::File;
use std::io::{prelude::*, BufReader, Error, ErrorKind};

fn parser<'a>(file: &File, facts: &'a Facts) -> Result<Rules<'a>, Error> {
    let reader = BufReader::new(file);
    let mut rules = Rules::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            match line.trim().chars().next() {
                Some('A'..='Z') | Some('(') | Some('!') => rules.set_rule(&facts, &line)?,
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
    Ok(rules)
}

fn solver(file: &File) -> Result<Facts, Error> {
    let facts = Facts::new();
    let mut rules = parser(file, &facts)?;
    rules.to_reverse_polish_notation()?;
    rules.print();
    rules.solve()?;
    checker::solved_queries(&facts)?;
    Ok(facts)
}

fn expert_system(file: File) -> Result<Facts, Error> {
    let solved_facts: Facts = solver(&file)?;
    print_solved_to_file(OUTPUT_FILE, &solved_facts)?;
    Ok(solved_facts)
}

pub fn run_ep(filename: &str) {
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
