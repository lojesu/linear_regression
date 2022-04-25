use std::env;
use std::fs;
use std::process;
use plotters::prelude::*;

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

    let values: Vec<&str> = content_file.trim().split(',').collect();
    let mut teta_zero: f32 = 0.0;
    let mut teta_one: f32 = 0.0;
    let mut max_x: f32 = 0.0;
    let mut max_y: f32 = 0.0;
    let km_f32: f32;
    if values.len() == 4 {
        match values[0].parse::<f32>() {
            Ok(n) => teta_zero = n,
            Err(_) => (),
        }
        match values[1].parse::<f32>() {
            Ok(n) => teta_one = n,
            Err(_) => (),
        }
        match values[2].parse::<f32>() {
            Ok(n) => max_x = n,
            Err(_) => (),
        }
        match values[3].parse::<f32>() {
            Ok(n) => max_y = n,
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
    let root_area = BitMapBackend::new("graph.png", (900, 900))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Linear Regression", ("sans-serif", 40))
    .build_cartesian_2d(0..(max_x + 0.15 * max_x) as i32, 0..(max_y + 0.15 * max_y) as i32)
    .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
    LineSeries::new((0..=(max_x + 0.15 * max_x) as i32).map(|x| (x, (teta_zero + teta_one * (x as f32)) as i32)), &GREEN)
    ).unwrap();

    let mut kms = Vec::new();
    let mut prices = Vec::new();

    let content_file = match fs::read_to_string("../data.csv") {
        Ok(content_file) => content_file,
        Err(_) => {
            println!("something went wrong reading the file for draw the graph");
            process::exit(1);
        }
    };
    for line in content_file.trim().split('\n') {
        let line_split: Vec<&str> = line.split(',').collect(); 
        if line_split.len() == 2 {
            match line_split[0].parse::<f32>() {
                Ok(n) => kms.push(n),
                Err(_) => (),
            }
            match line_split[1].parse::<f32>() {
                Ok(n) => prices.push(n),
                Err(_) => (),
            }
        }
    }
    let mut data = Vec::new();
    let mut i: usize = 0;
    while i < kms.len() {
        data.push((kms[i] as i32, prices[i] as i32));
        i += 1;
    }
    ctx.draw_series(data.iter().map(|point| Circle::new(*point, 2, &RED)))
        .unwrap();
}
