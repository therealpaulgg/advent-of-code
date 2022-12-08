use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_values(ip: String) -> Result<(i32, i32, i32, i32), String> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let matches = re.captures(&ip);
    if let Some(things) = matches {
        let n1 = things
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let n2 = things
            .get(2)
            .unwrap()
            .as_str()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let n3 = things
            .get(3)
            .unwrap()
            .as_str()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let n4 = things
            .get(4)
            .unwrap()
            .as_str()
            .to_string()
            .parse::<i32>()
            .unwrap();
        return Ok((n1, n2, n3, n4));
    }
    Err("Could not extract data from regex.".to_string())
}

fn part1() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut count = 0;
        for line in lines {
            if let Ok(ip) = line {
                if let Ok((n1, n2, n3, n4)) = get_values(ip) {
                    if (n1 <= n3 && n2 >= n4) || (n3 <= n1 && n4 >= n2) {
                        count += 1;
                    }
                }
            } else {
                eprintln!("Error while reading file...");
                return;
            }
        }
        println!("{}", count);
    } else {
        eprintln!("Could not read file.");
    }
}

fn part2() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut count = 0;
        for line in lines {
            if let Ok(ip) = line {
                if let Ok((n1, n2, n3, n4)) = get_values(ip) {
                    if (n1 <= n3 && n2 >= n3) || (n3 <= n1 && n4 >= n1) {
                        count += 1;
                    }
                }
            } else {
                eprintln!("Error while reading file...");
                return;
            }
        }
        println!("{}", count);
    } else {
        eprintln!("Could not read file.");
    }
}

fn main() {
    part1();
    part2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
