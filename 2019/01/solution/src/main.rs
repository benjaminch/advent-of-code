use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    writeln!(
        io::stdout(),
        "Total fuel required: {}",
        compute_fuel_requirement(&input, false)
    )?;

    // Part 2
    writeln!(
        io::stdout(),
        "Total fuel required (with fuel weight): {}",
        compute_fuel_requirement(&input, true)
    )?;

    Ok(())
}

fn compute_fuel_requirement(input_modules: &str, compute_fuel_for_fuel: bool) -> u32 {
    return input_modules
        .lines()
        .map(|m| (compute_fuel_for_mass(m.parse::<u32>().unwrap(), compute_fuel_for_fuel)))
        .sum();
}

fn compute_fuel_for_mass(module: u32, compute_fuel_for_fuel: bool) -> u32 {
    let mut fuel: u32 = 0;
    let mut mass: u32 = module;
    let fuel_for_mass = |weight: u32| -> u32 {
        let fuel: i32 = (weight / 3) as i32 - 2;
        if fuel <= 0 {
            return 0;
        }
        fuel as u32
    };

    loop {
        mass = fuel_for_mass(mass);
        fuel += mass;

        if !compute_fuel_for_fuel || mass <= 0 {
            break;
        }
    }

    fuel
}
