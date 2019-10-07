pub mod common;
use lib::expert_system;

use std::io::Error;

#[test]
fn empty_file() -> Result<(), Error> {
    expert_system::run("");
    Ok(())
}
#[test]
fn file_doesnt_exist() -> Result<(), Error> {
    expert_system::run("Voldemort");
    Ok(())
}
#[test]
fn is_directory() -> Result<(), Error> {
    expert_system::run("src");
    Ok(())
}
#[test]
fn no_rights() -> Result<(), Error> {
    expert_system::run("/.file");
    Ok(())
}
