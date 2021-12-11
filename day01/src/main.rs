use std::collections::HashSet;
use std::fs;
use std::io::{self, prelude::*};

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let sequence: Vec<i32> = reader.lines().map(|line| line.unwrap().parse().unwrap()).collect();

    let mut frequency = 0;
    let mut frequencies_seen: HashSet<i32> = HashSet::from([0]);

    for &delta in &sequence {
        frequency += delta;
    }
    println!("Frequency after first 100 runs: {}", frequency);

    frequency = 0;
    'outer: loop {
        for &delta in &sequence {
            frequency += delta;
            if frequencies_seen.contains(&frequency) {
                println!("First duplicate frequency found: {}", frequency);
                break 'outer;
            } else {
                frequencies_seen.insert(frequency);
            }
        }
    }
    
    Ok(())
}
