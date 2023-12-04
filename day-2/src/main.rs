use std::{fs, error::Error, cmp};

#[derive(Debug)]
struct Grab {
    r: u32,
    g: u32,
    b: u32,
}

impl Grab {
    fn new_from(s: &str) -> Self {
        let color_data = s.split(", ").map(|c_data| {
            let mut iter = c_data.split(" ");
            let num: u32 = iter.next().unwrap().parse().unwrap();
            let color = iter.next().unwrap();
            (num, color)
        });

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for (num, color) in color_data {
            match color {
                "red" => r = num,
                "green" => g = num,
                "blue" => b = num,
                _ => unreachable!("Should not have colors other than RGB, but found {}", color),
            }
        }

        Self { r, g, b }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let games: Vec<&str> = input.split("\n").filter(|s| s.len() > 0).collect();

    let goal = Grab { r: 12, g: 13, b: 14 };

    let mut valid_ids = Vec::with_capacity(games.len());
    let mut powers = Vec::with_capacity(games.len());
    'games: for g in games {
        let mut divider = g.split(": ");
        let g_id = divider.next().unwrap().split(" ").nth(1).unwrap().parse::<u32>()?;
        let data = divider.next().unwrap();
        let grabs: Vec<Grab> = data.split("; ").map(|g| Grab::new_from(g)).collect();

        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;
        for grab in grabs.iter() {
            max_r = cmp::max(max_r, grab.r);
            max_g = cmp::max(max_g, grab.g);
            max_b = cmp::max(max_b, grab.b);
        }
        let power = max_r * max_g * max_b;
        powers.push(power);

        for grab in grabs.iter() {
            if grab.r > goal.r
            || grab.g > goal.g
            || grab.b > goal.b {
                continue 'games;
            }
        }
        valid_ids.push(g_id);
    }

    println!("answer part 1: {}", valid_ids.iter().sum::<u32>());
    println!("answer part 2: {}", powers.iter().sum::<u32>());

    Ok(())
}
