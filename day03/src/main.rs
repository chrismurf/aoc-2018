use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs;
use std::str::FromStr;
use std::io::{self, prelude::*};

struct Claim {
    id : u32,
    position: [u32; 2],
    size: [u32; 2],
}

impl Claim {
    pub fn area(&self) -> u32 {
        self.size[0] * self.size[1]
    }

    pub fn claimed(&self) -> HashSet<(u32,u32)> {
        let mut claimed = HashSet::with_capacity(self.area() as usize);
        for row in self.position[0]..(self.position[0] + self.size[0]) {
            let cells = (self.position[1]..self.position[1] + self.size[1])
                .map(|col| (row, col));
            claimed.extend(cells);
        }
        claimed
    }
}

#[derive(Debug)]
pub struct ParseError;
impl FromStr for Claim {
    type Err = ParseError;

    // Parse '#18 @ 373,129: 29x21\n'
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts : Vec<&str> = s.trim().split_whitespace().collect();
        let id = parts[0][1..].parse().unwrap();
        let position = parts[2][..parts[2].len()-1]
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>()
            .try_into().unwrap();
        let size = parts[3]
            .split("x")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>()
            .try_into().unwrap();
        Ok(Claim { id, position, size })
    }
}

fn main() -> io::Result<()> {
    // Load Data and parse claims
    let file = fs::File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let claims : Vec<Claim> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // Find claims for each square
    let mut counts: HashMap<(u32, u32), HashSet<u32>> = HashMap::new();
    for claim in &claims {
        for square in claim.claimed() {
            counts.entry(square).or_insert(HashSet::new()).insert(claim.id);
        }
    }
    
    // Find squares with >1 claim (Part 1)
    let contested = counts.iter().filter(|(_, v)| v.len() > 1).count();
    println!("{} contested squares", contested);

    // Find uncontested claims (Part 2)
    let mut uncontested_ids: HashSet<u32> = HashSet::from_iter(claims.iter().map(|x| x.id));
    for (_, square_claims) in counts {
        if square_claims.len() > 1 {
            for sc in square_claims {
                uncontested_ids.remove(&sc);
            }
        }
    }
    println!("Uncontested IDs are {:?}", uncontested_ids);

    Ok(())
}
