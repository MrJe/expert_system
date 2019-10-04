// const OUTPUT_FILE: &str = "RESULT.txt";

use crate::facts::{Facts, Fact};
use crate::rules::Rules;
use crate::solver;
use crate::print;

use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader, Error, ErrorKind};

fn parser<'a>(file: File, facts: &'a Facts) -> Result<Rules<'a>, Error> {
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

fn queries_of_parsed(facts: &Facts) -> Vec<&Fact> {
    let mut queries = Vec::new();
    for fact in facts.fact_arr.iter() {
        if fact.queried.get() == true {
            queries.push(fact);
        }
    }
    queries
}

fn expert_system(file: File) -> Result<Vec<Fact>, Error> {
    let facts = Facts::new();
    let mut rules = parser(file, &facts)?;
    rules.to_reverse_polish_notation()?;
    let queries = queries_of_parsed(&facts);
    let solved_queries: Vec<Fact> = solver::solve(queries, rules)?;
    Ok(solved_queries)
}

fn expert_system_wrapper(file: File) {
    match expert_system(file) {
        Ok(solved_queries) => {
            // print::solved_to_file(OUTPUT_FILE, &solved_queries);
            print::results(&solved_queries);
        },
        Err(error) => println!(
            "Oops, something went wrong, shutting program down.\nError:\n{:#?}",
            error
        ),
    }
}

pub fn run_ep(filename: &str) {
    if Path::new(filename).is_dir() == true {
        println!("open: {}: Is a directory", filename);
    } else {
        match File::open(filename) {
            Ok(file)    => expert_system_wrapper(file),
            Err(error)  => println!("open: {}: {:#?}", filename, error),
        };
    }
}