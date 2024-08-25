use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rand::seq::SliceRandom;
use rand::thread_rng;

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

fn main() -> io::Result<()> {
    let owners = read_lines("input/owners.txt")?;
    let mut options = read_lines("input/options.txt")?;

    if owners.len() != options.len() {
        eprintln!(
            "Error: The number of owners ({}) does not match the number of options ({}).",
            owners.len(),
            options.len()
        );
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Mismatched file lengths",
        ));
    }

    // Shuffle the options
    let mut rng = thread_rng();
    options.shuffle(&mut rng);

    // Match players with options and print in the desired format
    for (owner, driver) in owners.iter().zip(options.iter()) {
        println!("{owner}: {driver}");
    }

    Ok(())
}
