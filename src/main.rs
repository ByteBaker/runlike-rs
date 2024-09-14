mod args;
mod model;
mod options;
mod util;

use options::Options;
use std::convert::Infallible;

fn main() -> Result<(), Infallible> {
    match Options::try_new() {
        Ok(options) => {
            options.print();
        }
        Err(e) => eprintln!("Error: {e}"),
    }

    Ok(())
}
