use crate::facts::Facts;

use std::fs::File;
use std::io::{prelude::*, Error};

pub fn print_results(facts: Facts) {
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

pub fn print_solved_to_file(fname: &str, facts: &Facts) -> Result<(), Error> {
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
