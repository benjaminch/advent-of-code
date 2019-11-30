#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::{self, Read, Write};
use std::vec::Vec;
use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let claims = get_claims(&input);
    writeln!(io::stdout(), "{:?}", claims);
}

#[derive(Debug)]
struct Claim {
    id: String,
    position_x: u32,
    position_y: u32,
    width: u32,
    height: u32,
}

fn get_claims(input: &str) -> Vec<Claim> {
    let mut claims: Vec<Claim> = Vec::new();

    for line in input.lines() {
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
        }
    } 
    return claims;
}
