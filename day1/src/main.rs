use std::fs;

fn calculate_fuel(mass: f32) -> f32 {
    // to find the fuel required for a module, take its mass, divide by three,
    // round down, and subtract 2.
    (mass / 3.0).floor() - 2.0
}

fn calculate_module_fuel(mass: f32) -> f32 {
    let mut total = 0.0;
    let mut _mass = mass;
    loop {
        let fuel = calculate_fuel(_mass);
        if fuel <= 0.0 {
            return total;
        };
        total += fuel;
        _mass = fuel;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read_to_string("input.txt")?;
    let result: f32 = input
        .lines()
        .map(|l| calculate_module_fuel(l.parse::<f32>().unwrap()))
        .sum();
    println!("Total: {}", result);
    Ok(())
}
