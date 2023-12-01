mod ant;
mod hive;
use hive::hive::{Colony, Hive};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].to_string().parse::<usize>().unwrap();
    let small_map = "./hiveum_map_small.txt";
    let medium_map = "./hiveum_map_medium.txt";
    let colonies = serialize(medium_map).unwrap();
    let mut hive = Hive::new(colonies);
    for i in 0..=n {
        hive.add_ant(&i);
    }
    for i in 0..=10000 {
        // Simulating "10,000" ant moves
        // todo: check hive.ants to see if any ants are left, end if so, or check specific ants move counter
        hive.simulate();
    }
}

/// Takes a world map and converts it into a vector of colonies
fn serialize(file_name: &str) -> Result<Vec<Colony>> {
    let file = File::open(file_name)?;
    let lines = BufReader::new(file);

    let mut colonies: Vec<Colony> = Vec::new();
    // todo: add encoder for cardinal directions
    for line in lines.lines() {
        if let Ok(content) = line {
            let words: Vec<&str> = content.split_whitespace().collect();
            //println!("k {:?}", words);
            let location = words[0].to_string(); // colony name
            let mut tunnels: Vec<String> = Vec::new();

            for i in 1..words.len() {
                if let Some(index) = words[i].find('=') {
                    let (_, tunnel) = words[i].split_at(index);
                    tunnels.push(tunnel.to_string());
                }
            }
            colonies.push(Colony::new(location, tunnels));
        }
    }
    Ok(colonies)
}

// todo: take encoder and revert to txt stream
fn deserialize() {}
