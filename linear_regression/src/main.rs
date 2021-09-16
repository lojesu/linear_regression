use std::fs;
use std::env;

mod calcul;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content_file = fs::read_to_string(filename)
        .expect("something went wrong reading the file");

    let mut kms = Vec::new();
    let mut prices = Vec::new();

    for line in content_file.trim().split('\n') {
        let line_split: Vec<&str> = line.split(',').collect(); 
        match line_split[0].parse::<f32>() {
            Ok(n) => kms.push(n),
            Err(_) => (),
        }
        match line_split[1].parse::<f32>() {
            Ok(n) => prices.push(n),
            Err(_) => (),
        }
    }
    println!("{:?}", kms);
    println!("{:?}", prices);
    
    let teta_one = calcul::teta_one(&kms, &prices);
    let teta_zero = calcul::teta_zero(&kms, &prices, teta_one);
    println!("t1 => {}\nt0 => {}", teta_one, teta_zero);
}
