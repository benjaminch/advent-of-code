use std::collections::HashMap;
use std::io::{self, Error, Read, Write};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn has_all_required_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid(&self) -> bool {
        if !self.has_all_required_fields() {
            return false;
        }

        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if !(match self.byr.clone().unwrap().parse::<u32>() {
            Ok(v) => (1920..=2002).contains(&v),
            _ => false,
        }) {
            return false;
        }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if !(match self.iyr.clone().unwrap().parse::<u32>() {
            Ok(v) => (2010..=2020).contains(&v),
            _ => false,
        }) {
            return false;
        }

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if !(match self.eyr.clone().unwrap().parse::<u32>() {
            Ok(v) => (2020..=2030).contains(&v),
            _ => false,
        }) {
            return false;
        }

        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        let hgt = self.hgt.as_ref().unwrap().clone();
        if hgt.contains("cm") {
            if !match hgt.replace("cm", "").parse::<u32>() {
                Ok(v) => (150..=193).contains(&v),
                _ => false,
            } {
                return false;
            }
        } else if hgt.contains("in") {
            if !match hgt.replace("in", "").parse::<u32>() {
                Ok(v) => (59..=76).contains(&v),
                _ => false,
            } {
                return false;
            }
        } else {
            return false;
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let hcl = self.hcl.as_ref().unwrap().clone();
        if !hcl.starts_with('#') {
            return false;
        }
        for c in hcl.chars().skip(1) {
            if !c.is_ascii_digit() && c < 'a' || c > 'f' {
                return false;
            }
        }

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        let ecl = self.ecl.as_ref().unwrap().clone();
        if ecl != "amb"
            && ecl != "blu"
            && ecl != "brn"
            && ecl != "gry"
            && ecl != "grn"
            && ecl != "hzl"
            && ecl != "oth"
        {
            return false;
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        let mut pid_length: usize = 0;
        for c in self.pid.as_ref().unwrap().clone().chars() {
            if !c.is_ascii_digit() {
                return false;
            }

            pid_length += 1;
        }
        if pid_length != 9 {
            return false;
        }

        // cid (Country ID) - ignored, missing or not.

        true
    }
}

impl FromStr for Passport {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = HashMap::new();
        let mut passport = Passport::new();
        let fields_raw = s.replace(" ", "\n");

        for line in fields_raw.lines() {
            if let Some((field_name, field_value)) = split_once(line.trim(), ':') {
                fields.insert(field_name.to_string(), field_value.to_string());
            }
        }

        if let Some(byr) = fields.get("byr") {
            passport.byr = Some(byr.clone());
        }
        if let Some(iyr) = fields.get("iyr") {
            passport.iyr = Some(iyr.clone());
        }
        if let Some(eyr) = fields.get("eyr") {
            passport.eyr = Some(eyr.clone());
        }
        if let Some(hgt) = fields.get("hgt") {
            passport.hgt = Some(hgt.clone());
        }
        if let Some(hcl) = fields.get("hcl") {
            passport.hcl = Some(hcl.clone());
        }
        if let Some(ecl) = fields.get("ecl") {
            passport.ecl = Some(ecl.clone());
        }
        if let Some(pid) = fields.get("pid") {
            passport.pid = Some(pid.clone());
        }
        if let Some(cid) = fields.get("cid") {
            passport.cid = Some(cid.clone());
        }

        Ok(passport)
    }
}

fn split_once(in_string: &str, c: char) -> Option<(&str, &str)> {
    let mut splitter = in_string.splitn(2, c);
    let first = splitter.next();
    let second = splitter.next();

    if first.is_none() || second.is_none() {
        None
    } else {
        Some((first.unwrap(), second.unwrap()))
    }
}

fn get_passports(s: &str) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();

    for raw_entry in s.split("\n\n") {
        if let Ok(passport) = Passport::from_str(raw_entry) {
            passports.push(passport);
        }
    }

    passports
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let part_1_valid_passports = get_passports(&input)
        .iter()
        .filter(|p| p.has_all_required_fields())
        .count();
    writeln!(
        io::stdout(),
        "Part - 1 / Valid passports: {}",
        part_1_valid_passports
    )?;

    // Part 2
    let part_2_valid_passports = get_passports(&input)
        .iter()
        .filter(|p| p.is_valid())
        .count();
    writeln!(
        io::stdout(),
        "Part - 2 / Valid passports: {}",
        part_2_valid_passports
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {}
