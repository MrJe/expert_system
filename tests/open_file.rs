pub mod common;
use lib::expert_system::run_ep;

use std::io::Error;

#[test]
fn file_doesnt_exist() -> Result<(), Error> {
    run_ep("Voldemort");
    Ok(())
}
