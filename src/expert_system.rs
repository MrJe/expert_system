const OUTPUT_FILE: &str = "RESULT.txt";

use crate::checker;
use crate::facts::Facts;
use crate::rules::Rules;
use crate::print::{print_solved_to_file, print_results};

use std::fs::File;
use std::path::Path;
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

fn expert_system(file: File) {
    match solver(&file) {
        Ok(facts)   => {
            print_solved_to_file(OUTPUT_FILE, &facts);
            print_results(facts);
        },
        Err(error)  => println!(
            "Oops, something went wrong, shutting program down.\nError:\n{:#?}",
            error
        ),
    }
}

fn open_file(filename: &str) {
    match File::open(filename) {
        Ok(file)    => expert_system(file),
        Err(error)  => println!("open: {}: {:#?}", filename, error),
    };
}

pub fn run_ep(filename: &str) {
    if Path::new(filename).is_dir() == true {
        println!("open: {}: Is a directory", filename);
    } else {
        open_file(filename);
    }
}
