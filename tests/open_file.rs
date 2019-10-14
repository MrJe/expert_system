pub mod common;
use lib::expert_system;
use lib::options::Options;

use std::io::Error;

#[test]
fn empty_file() -> Result<(), Error> {
    expert_system::run("", &Options::new());
    Ok(())
}
#[test]
fn file_doesnt_exist() -> Result<(), Error> {
    expert_system::run("Voldemort", &Options::new());
    Ok(())
}
#[test]
fn is_directory() -> Result<(), Error> {
    expert_system::run("src", &Options::new());
    Ok(())
}
#[test]
fn no_rights() -> Result<(), Error> {
    expert_system::run("/.file", &Options::new());
    Ok(())
}
