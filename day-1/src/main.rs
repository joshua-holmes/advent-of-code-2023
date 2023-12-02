use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let lines = input.split("\n").filter(|l| l.len() > 0);

    let mut calibration_values = Vec::new();
    for l in lines {
        let mut forward = l.chars();
        let mut reverse = l.chars().rev();

        let digit_1 = forward.find(|c| c.is_digit(10)).unwrap();
        let digit_2 = reverse.find(|c| c.is_digit(10)).unwrap();

        let value = String::from_iter([digit_1, digit_2]).parse::<u32>()?;
        calibration_values.push(value);
    }

    println!("{}", calibration_values.iter().sum::<u32>());

    Ok(())
}
