use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let km = match args.len() {
       2 => args[1].to_string(),
       _ => {
           println!("Program accept one argument");
           process::exit(0)
        }
    };
    let content_file = fs::read_to_string("../../my_data.csv")
        .expect("Something went wrong reading the file");

    let tetas: Vec<&str> = content_file.trim().split(',').collect();
    let mut teta_zero: f32 = 0.0;
    let mut teta_one: f32 = 0.0;
    let km_f32: f32;
    if tetas.len() == 2 {
        match tetas[0].parse::<f32>() {
            Ok(n) => teta_zero = n,
            Err(_) => (),
        }
        match tetas[1].parse::<f32>() {
            Ok(n) => teta_one = n,
            Err(_) => (),
        }
    }
    match km.parse::<f32>() {
        Ok(n) => km_f32 = n,
        Err(_) => km_f32 = 0.0,
    };

    let mut result = teta_zero + teta_one * km_f32;
    match result {
       x if x < 0.0 => result = 0.0,
       _ => (),
    }
    println!("estimated price is {}", result);
}
