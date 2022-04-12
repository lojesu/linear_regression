use std::env;
use std::fs;
use std::process;

fn is_number(s: &String) -> bool {
    for (i, c) in s.chars().enumerate() {
        if c.is_digit(10) == false {
            if c == '-' && i == 0 {
                continue;
            }
            return false
        }
    }
    true
}

fn print_helper() {
    println!("Usage: cargo run [FILE] [KM]");
    println!("[FILE]: the file that program need for correctly estimate the price");
    println!("[KM]: the mileage or you want to estimate the price based on the training of the program");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
       3 => (),
       _ => {
           print_helper();
           process::exit(0)
        }
    };
    let km = match is_number(&args[2]) {
        true => &args[2],
        _ => {
            println!("km must be a number");
            process::exit(0);
        }
    };
    let content_file = match fs::read_to_string(&args[1]) {
        Ok(content_file) => content_file,
        Err(_) => {
            println!("Something went wrong reading the file");
            process::exit(0);
        }
    };

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
