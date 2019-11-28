#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::{self, BufRead};
use std::vec::Vec;
use std::fs::File;
use std::path::Path;
use regex::Regex;

fn main() {
    // Part 1
    let claims = get_claims(Path::new("../input.txt"));
    println!("{:?}", claims);
}

#[derive(Debug)]
struct Claim {
    id: String,
    position_x: u32,
    position_y: u32,
    width: u32,
    height: u32,
}

fn get_claims(file: &Path) -> Vec<Claim> {
    let mut claims: Vec<Claim> = Vec::new();
    if let Ok(lines) = read_lines(file) {
		for l in lines {
            if let Ok(line) = l {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^(?P<id>[\S]+) @ (?P<position_x>[\d]+),(?P<position_y>[\d]+): (?P<width>[\d]+)x(?P<height>[\d]+)").unwrap();
                }
                if let Some(caps) = RE.captures(&line) {
                    let claim = Claim{
                        id: caps["id"].parse().unwrap(),
                        position_x: caps["position_x"].parse().unwrap(),
                        position_y: caps["position_y"].parse().unwrap(),
                        width: caps["width"].parse().unwrap(),
                        height: caps["height"].parse().unwrap(),
                    };
                    claims.push(claim);
                };
            }
        }
    } 
    return claims;
}

fn read_lines(filename: &Path) -> std::result::Result<std::io::Lines<std::io::BufReader<std::fs::File>>, std::io::Error> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
