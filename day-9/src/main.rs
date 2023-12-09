use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let history: Vec<Vec<i64>> = input
        .split('\n')
        .filter(|l| l.len() > 0)
        .map(|l| l.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut values_right_of_history = Vec::with_capacity(history.len());
    for h in history.iter() {
        let len = h.len();
        let mut extrapolation = vec![h.clone()];
        let mut cur_extr = h;
        let mut is_zeros = h.iter().all(|n| *n == 0);

        while !is_zeros {
            let mut new_extr: Vec<i64> = Vec::with_capacity(len);
            let mut prev_num = None;

            for num in cur_extr.iter() {
                if let Some(prev_num) = prev_num {
                    let diff = num - prev_num;
                    new_extr.push(diff);
                }
                prev_num = Some(num);
            }

            is_zeros = new_extr.iter().all(|n| *n == 0);
            extrapolation.push(new_extr);
            cur_extr = extrapolation.last().unwrap();
        }
        println!("{:?}", extrapolation);

        // find new value right of history
        let mut num_to_add = 0;
        for (i, extr) in extrapolation.iter_mut().rev().enumerate() {
            if i == 0 {
                continue;
            }

            num_to_add += extr.last().unwrap();
        }
        println!("new num {}\n", num_to_add);
        values_right_of_history.push(num_to_add);
    }

    println!("answer {}", values_right_of_history.iter().sum::<i64>());

    Ok(())
}
