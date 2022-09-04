use std::io::{BufRead, BufReader};

mod function;
mod generated;

use generated::*;

fn main() {
    if let Err(e) = process() {
        let err = generated::Error {
            message: e.to_string(),
        };
        eprintln!("{}", serde_json::to_string(&err).unwrap());
    }
}

fn process() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdin = BufReader::new(std::io::stdin()).lines();

    while let Some(Ok(line)) = stdin.next() {
        let input: InputRecord = serde_json::from_str(&line)?;
        let output = function::process(input)?;
        println!("{}", serde_json::to_string_pretty(&output)?);
    }
    Ok(())
}
