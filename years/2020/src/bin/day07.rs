use std::collections::HashMap;
use std::io::{self, Error, Read, Write};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Color {
    v: String,
}

impl Color {
    fn new(v: String) -> Color {
        Color { v }
    }
}

#[derive(Debug, Clone)]
struct FinishType {
    v: String,
}

impl FinishType {
    fn new(v: String) -> FinishType {
        FinishType { v }
    }
}

#[derive(Debug, Clone)]
struct BagSlot {
    b: Bag,
    count: u32,
}

impl BagSlot {
    fn new(b: Bag, count: u32) -> BagSlot {
        BagSlot { b, count }
    }
}

#[derive(Debug, Clone)]
struct Bag {
    c: Color,
    t: FinishType,
}

impl Bag {
    fn new(c: Color, t: FinishType) -> Bag {
        Bag { c, t }
    }

    fn name(&self) -> String {
        [self.t.v.as_str(), self.c.v.as_str()].join(" ")
    }
}

impl FromStr for Bag {
    type Err = Error;

    fn from_str(input: &str) -> Result<Bag, Error> {
        if let Some((finish_type, color)) = input.split_once(" ") {
            return Ok(Bag::new(
                Color::new(color.trim().to_string()),
                FinishType::new(finish_type.trim().to_string()),
            ));
        }

        Err(Error::other("cannot parse"))
    }
}

#[derive(Debug)]
struct Rules {
    r: HashMap<String, Vec<BagSlot>>,
}

impl Rules {
    fn new() -> Rules {
        Rules { r: HashMap::new() }
    }

    fn add(&mut self, b: &Bag, s: Option<&BagSlot>) {
        if let Some(v) = self.r.get_mut(b.name().as_str()) {
            if s.is_some() {
                (*v).push(s.unwrap().clone());
            }
            return;
        }

        let slot_to_add = if s.is_none() {
            Vec::new()
        } else {
            vec![s.unwrap().clone()]
        };
        self.r.insert(b.name(), slot_to_add);
    }

    fn bag_can_contain(&self, from: &Bag, to: &Bag) -> bool {
        if let Some(subs) = self.r.get(&from.name()) {
            for sub in subs {
                if (sub.b.c.v == *to.c.v && sub.b.t.v == *to.t.v)
                    || self.bag_can_contain(&sub.b, to)
                {
                    return true;
                }
            }
        }

        false
    }

    fn bag_can_contain_count(&self, to: &Bag) -> usize {
        self.r
            .keys()
            .map(|k| Bag::from_str(k).unwrap())
            .filter(|b| self.bag_can_contain(b, to))
            .count()
    }

    fn bag_count(&self, b: &Bag) -> usize {
        if let Some(subs) = self.r.get(&b.name()) {
            if subs.is_empty() {
                return 1;
            }
            return 1 + subs
                .iter()
                .map(|s| self.bag_count(&s.b) * s.count as usize)
                .sum::<usize>();
        }

        0
    }
}

impl FromStr for Rules {
    type Err = Error;

    fn from_str(input: &str) -> Result<Rules, Error> {
        let mut rules = Rules::new();

        for line in input.lines() {
            if let Some((bag_raw_str, contained_bags_raw_str)) = line.split_once("contain") {
                let bag: Bag = Bag::from_str(
                    bag_raw_str
                        .replace(".", "")
                        .replace(",", "")
                        .replace("bags", "")
                        .replace("bag", "")
                        .trim(),
                )?;
                rules.add(&bag, None);

                for contained_bag_raw_str in contained_bags_raw_str.split(',') {
                    if contained_bag_raw_str.trim() == "no other bags" {
                        break;
                    }
                    if let Some((count, sub_bag)) = contained_bag_raw_str.trim().split_once(" ") {
                        if sub_bag.starts_with("other bags.") {
                            break;
                        }
                        rules.add(
                            &bag,
                            Some(&BagSlot::new(
                                Bag::from_str(
                                    sub_bag
                                        .replace(".", "")
                                        .replace(",", "")
                                        .replace("bags", "")
                                        .replace("bag", "")
                                        .trim(),
                                )?,
                                count.parse::<u32>().unwrap(),
                            )),
                        );
                    }
                }
            }
        }

        Ok(rules)
    }
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // TODO: Refactor from the ground up this day, everything is ugly and needs to be
    // done properly

    // Part 1
    let part_1_rules = Rules::from_str(&input)?;
    let part_1_count_can_contain = part_1_rules.bag_can_contain_count(&Bag::new(
        Color::new("gold".to_string()),
        FinishType::new("shiny".to_string()),
    ));
    writeln!(
        io::stdout(),
        "Part - 1 / Can contain shiny gold bag count: {:?}",
        part_1_count_can_contain
    )?;

    // Part 2
    let part_2_rules = Rules::from_str(&input)?;
    let part_2_bags_count = part_2_rules.bag_count(&Bag::new(
        Color::new("gold".to_string()),
        FinishType::new("shiny".to_string()),
    ));
    writeln!(
        io::stdout(),
        "Part - 2 / Shiny gold bag can contain: {:?}",
        part_2_bags_count - 1
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {}
