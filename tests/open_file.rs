pub mod common;
use lib::expert_system::run_ep;

use std::io::Error;

#[test]
fn empty_file() -> Result<(), Error> {
    run_ep("");
    Ok(())
}#[test]
fn file_doesnt_exist() -> Result<(), Error> {
    run_ep("Voldemort");
    Ok(())
}
#[test]
fn is_directory() -> Result<(), Error> {
    run_ep("src");
    Ok(())
}
#[test]
fn no_rights() -> Result<(), Error> {
    run_ep("/.file");
    Ok(())
}
