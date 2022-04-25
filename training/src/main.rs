use std::{env, process};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;

mod parse;
mod calcul;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = parse::filename(&args);
    let content_file = match fs::read_to_string(filename) {
        Ok(content_file) => content_file,
        Err(_) => {
            println!("something went wrong reading the file");
            process::exit(1);
        }
    };

    let mut kms = Vec::new();
    let mut prices = Vec::new();

    let content_file = parse::content_file(content_file);
    for line in content_file.trim().split('\n') {
        let line_split: Vec<&str> = line.split(',').collect(); 
        if line_split.len() == 2 {
            if line_split[0].len() > 0 {
                match line_split[0].parse::<f32>() {
                    Ok(n) => kms.push(n),
                    Err(_) => (),
                }
            } else {
                println!("Something went wrong when reading the file");
                process::exit(1);
            }
            if line_split[1].len() > 0 {
                match line_split[1].parse::<f32>() {
                    Ok(n) => prices.push(n),
                    Err(_) => (),
                }
            } else {
                println!("Something went wrong when reading the file");
                process::exit(1);
            }
        }
    }
    parse::check(&kms, &prices);
    
    let teta_one = calcul::teta_one(&kms, &prices);
    let teta_zero = calcul::teta_zero(&kms, &prices, teta_one);
    let max_x = kms.iter().copied().fold(f32::NAN, f32::max);
    let max_y = prices.iter().copied().fold(f32::NAN, f32::max);

    let mut f = File::create("../my_data.csv")?;
    f.write(format!("{},{},{},{}\n", teta_zero.to_string(), teta_one.to_string(), max_x.to_string(), max_y.to_string()).as_bytes())?;
    Ok(())
}
