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
    let players = read_lines("input/players.txt")?;
    let mut drivers = read_lines("input/drivers.txt")?;

    if players.len() != drivers.len() {
        eprintln!("Both files must contain the same number of names.");
        return Ok(());
    }

    // Shuffle the drivers
    let mut rng = thread_rng();
    drivers.shuffle(&mut rng);

    // Match players with drivers and print in the desired format
    for (player, driver) in players.iter().zip(drivers.iter()) {
        println!("{driver} - {player}");
    }

    Ok(())
}
