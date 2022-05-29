#![deny(clippy::all)]

use std::io;
use std::path::Path;

mod days;

pub use days::day_1;

/// An abstract error enum to handle errors.
#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}

/// A trait to run a daily solution. Each daily solution could have an input file to use.
/// This input file can be found into the *aoc_inputs* directory stored into current directory.
pub trait DailySolution {
    const DAY_NUMBER: u8;

    /// Run the solution over provided input path.
    fn run_solution<P>(input_path: P) -> Result<(), Error>
    where
        P: AsRef<Path>;

    /// Run the solution over default input path.
    fn run() -> Result<(), Error> {
        let default_input_path = std::env::current_dir()
            .unwrap()
            .join("aoc_inputs")
            .join(format!("daily_input_{}", Self::DAY_NUMBER));

        println!(
            "Start solution for day {} over input file '{}'",
            Self::DAY_NUMBER,
            default_input_path.display()
        );
        Self::run_solution(default_input_path)
    }
}
