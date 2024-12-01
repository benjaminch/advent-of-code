use std::collections::HashMap;
use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let orbits = build_orbits(&input);

    // Part 1
    let direct_and_indirect_count = get_direct_indirect_orbits(orbits.clone());
    writeln!(
        io::stdout(),
        "Directs + indirects orbits: {}",
        direct_and_indirect_count
    )?;

    // Part 2
    let min_transferts = get_min_transferts(orbits.clone(), "YOU", "SAN");
    writeln!(io::stdout(), "Min transferts: {}", min_transferts)?;

    Ok(())
}

fn get_direct_indirect_orbits(orbits: HashMap<&str, &str>) -> i32 {
    let mut count: i32 = 0;
    for (orbit, _ancestors) in orbits.clone().into_iter() {
        let mut next_orbit = orbits.get(&orbit);
        while next_orbit.is_some() {
            count += 1;
            next_orbit = orbits.get(next_orbit.unwrap());
        }
    }
    count
}

fn get_min_transferts(orbits: HashMap<&str, &str>, point_1: &str, point_2: &str) -> i32 {
    let mut point_1_path: HashMap<&str, i32> = HashMap::new();
    let mut transferts: i32 = 0;
    let mut next_orbit = orbits.get(&point_1);
    while next_orbit.is_some() {
        transferts += 1;
        point_1_path.insert(&next_orbit.unwrap(), transferts);
        next_orbit = orbits.get(next_orbit.unwrap());
    }
    transferts = 0;
    next_orbit = orbits.get(&point_2);
    while next_orbit.is_some() && !point_1_path.contains_key::<str>(&next_orbit.unwrap()) {
        transferts += 1;
        next_orbit = orbits.get(next_orbit.unwrap());
    }
    return transferts + point_1_path.get::<str>(&next_orbit.unwrap()).unwrap() - 1;
}

fn build_orbits(input: &str) -> HashMap<&str, &str> {
    let mut orbits: HashMap<&str, &str> = HashMap::new();
    for l in input.lines() {
        let s = l.split(')').collect::<Vec<&str>>();
        let (ancestor, orbit) = (s[0], s[1]);
        orbits.entry(orbit).or_insert(ancestor);
    }
    orbits
}
