use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn part1() {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut alpha_map: HashMap<char, i32> = HashMap::new();
    for (i, c) in alphabet.chars().enumerate() {
        alpha_map.insert(c, i as i32 + 1);
    }
    let mut priorities = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let length = ip.len();
                let compartment1 = &ip[0..length/2];
                let compartment2 = &ip[length/2..length];
                let mut map1: HashMap<char, bool> = HashMap::new();
                let mut char_match: Option<char> = None;
                for c in compartment1.chars() {
                    map1.insert(c, true);
                }
                for c in compartment2.chars() {
                    if map1.contains_key(&c) {
                        char_match = Some(c);
                        break;
                    }
                }
                if char_match.is_some() {
                    priorities += alpha_map[&char_match.unwrap()];
                }
            }
        }
    }
    println!("Part 1 Priorities: {}", priorities);
}

fn part2() {
    const GROUP_LENGTH: usize = 3;
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut alpha_map: HashMap<char, i32> = HashMap::new();
    for (i, c) in alphabet.chars().enumerate() {
        alpha_map.insert(c, i as i32 + 1);
    }
    let mut priorities = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        let mut array: [String; GROUP_LENGTH] = ["".to_string(), "".to_string(), "".to_string()];
        let mut i = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                array[i] = ip;
                i += 1;
                if i == GROUP_LENGTH {
                    i = 0;
                    let mut maps: Vec<HashMap<char, bool>> = Vec::new();
                    for j in 0..GROUP_LENGTH {
                        let mut map: HashMap<char, bool> = HashMap::new();
                        for c in array[j].chars() {
                            map.insert(c, true);
                        }
                        maps.push(map);
                    }
                    let mut in_all: bool;
                    for c in array[GROUP_LENGTH - 1].chars() {
                        in_all = true;
                        for j in 0..GROUP_LENGTH - 1 {
                            if !maps[j].contains_key(&c) {
                                in_all = false;
                                break;
                            }
                        }
                        if in_all {
                            priorities += alpha_map[&c];
                            break;
                        }
                    }
                }                
            }
        }
    }
    println!("Part 2 Priorities: {}", priorities);
}

fn main() {
    part1();
    part2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}