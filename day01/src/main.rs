use std::collections::HashSet;
use std::fs;
use std::io::{self, prelude::*};

fn find_first_duplicate_frequency(sequence: &Vec<i32>) -> i32 {
    let mut frequency = 0;
    let mut frequencies_seen: HashSet<i32> = HashSet::from([0]);
    loop {
        for &delta in sequence {
            frequency += delta;
            if frequencies_seen.contains(&frequency) {
                return frequency;
            } else {
                frequencies_seen.insert(frequency);
            }
        }
    }
}

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let sequence: Vec<i32> = reader.lines().map(|line| line.unwrap().parse().unwrap()).collect();

    println!("Frequency after first 100 runs: {}", sequence.iter().sum::<i32>());
    println!("First duplicate frequency found: {}", find_first_duplicate_frequency(&sequence));
    
    Ok(())
}
