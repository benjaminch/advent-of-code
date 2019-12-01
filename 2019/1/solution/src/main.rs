use std::io::{self, Read, Write};
use std::vec::Vec;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    // Part 1
    writeln!(io::stdout(), "Total fuel required: {}", compute_fuel_requirement(&input));

    // Part 2
    writeln!(io::stdout(), "Total fuel required (with fuel weight): {}", compute_fuel_requirement_with_fuel_weight(&input));
}

fn compute_fuel_requirement(input_modules: &str) -> u32 {
    return input_modules
        .lines()
        .map(|m| (get_required_fuel_for_module(m.parse::<u32>().unwrap())))
        .sum();
}

fn get_required_fuel_for_module(module_weight: u32) -> u32 {
    let fuel: i32 = (module_weight / 3) as i32 - 2;
    if fuel <= 0 {
        return 0;
    }
    return fuel as u32;
}

fn compute_fuel_requirement_with_fuel_weight(input_modules: &str) -> u32 {
    return input_modules
        .lines()
        .map(|m| (get_required_fuel_for_module_with_fuel_weight(m.parse::<u32>().unwrap())))
        .sum();
}

fn get_required_fuel_for_module_with_fuel_weight(module_weight: u32) -> u32 {
    let fuel: i32 = (module_weight / 3) as i32 - 2;
    if fuel <= 0 {
        return 0;
    }

    let mut sum_fuel: u32 = fuel as u32;
    let mut fuel_for_module = Vec::new();
    fuel_for_module.push(sum_fuel as i32);

    while let Some(fuel) = fuel_for_module.pop() {
        let f = (fuel / 3) as i32 - 2;
        if f > 0 {
            sum_fuel += f as u32;
            fuel_for_module.push(f);

        }
    }

    return sum_fuel;
}
