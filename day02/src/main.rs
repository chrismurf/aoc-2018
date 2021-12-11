use std::collections::{HashSet};
use std::fs;
use std::iter::Iterator;
use std::io::{self, prelude::*};

fn hamming_distance(str_a: &str, str_b: &str) -> usize {
    Iterator::zip(str_a.chars(), str_b.chars())
        .filter(|(x,y)| x != y)
        .count()
}

fn common_chars(str_a: &str, str_b: &str) -> String {
    Iterator::zip(str_a.chars(), str_b.chars())
        .filter(|(x,y)| x == y)
        .map(|(x,_)| x)
        .collect()
}

fn main() -> io::Result<()> {
    // Load Data
    let file = fs::File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let ids : Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // Solve part one - count IDs with at least one double or triple
    let mut doubles: u32 = 0;
    let mut triples: u32 = 0;
    for id in &ids {
        let chars: HashSet<char> = HashSet::from_iter(id.chars());
        let counts: Vec<u32> = chars.iter().map(|&c| id.matches(c).count() as u32).collect();
        if counts.contains(&2u32) { doubles += 1; }
        if counts.contains(&3u32) { triples += 1; }
    }
    println!("Double * Triple = {}", doubles * triples);

    // Solve part two - find two IDs with a hamming distance of one
    'outer: for i in 0..ids.len() {
        for j in (i+1)..ids.len() {
            let distance = hamming_distance( &ids[i], &ids[j]);
            if distance == 1 {
                println!("Hamming distance of 1 between {} and {}", &ids[i], &ids[j]);
                println!("Common base string of {}", common_chars(&ids[i], &ids[j]));
                break 'outer;
            }
        }
    }

    Ok(())
}
