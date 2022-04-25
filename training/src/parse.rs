use std::process;

pub fn filename(args: &Vec<String>) -> String {
    match args.len() {
        2 => args[1].to_string(),
        _ => {
            println!("Program need a csv file for training");
            process::exit(0)
        }
    }
}


pub fn content_file(s: String) -> String {
    let mut ret = String::new();

    for c in s.chars() {
        if c.is_digit(10) || c == ',' || c == '\n' {
            ret.push(c);
        }
    }
    ret
}


pub fn check(kms: &Vec<f32>, prices: &Vec<f32>) {
    if kms.len() == 0 || prices.len() == 0 {
        println!("your file countain any value");
        process::exit(0);
    }
}
