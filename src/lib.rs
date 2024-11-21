//! # lnecho
//! 
//! 
//! These Are the tools used to create an `echo` like command

/// Used to create the config that the `run` function needs.
/// 
/// * Implements a method called `from` that takes an `Iterator<Item=String>` as input.
/// * Only has one field: input: `Vec<String>`.
/// 
/// # Example
/// ```
/// let config = lnecho::Config::from(std::env::args());
/// ```
pub struct Config {
    input: Vec<String>,
}

impl Config {
    /// Using this function you can create an instance of Config from any T: Iterator<Item=String>
    pub fn from(mut input: impl Iterator<Item=String>) -> Config {
        input.next();
        Config {
            input: input.collect()
        }
    }
}

/// Runs the main programm
/// 
/// # Example
/// 
/// ```
/// let config = lnecho::Config::from(std::env::args());
/// 
/// lnecho::run(config);
/// ```
pub fn run(config: Config) {
    for input in config.input {
        print!("{} ", input);
    }
    println!();
}
