use std::io::{stderr, stdin, stdout, Write};

fn err(e: Box<dyn std::error::Error>, while_sth: &str) {
    let mut prep_str = "\r\n Error occured, ".to_owned();
    prep_str.push_str(while_sth);
    prep_str.push_str(&e.to_string());
    stderr().write(prep_str.as_bytes()).unwrap();
}

const CELSIUS_MUL: f64 = 1.8;
const CELSIUS_DRIFT: f64 = 32.;

fn cels_to_faren(faren: f64) -> f64 {
    (faren * CELSIUS_MUL) + CELSIUS_DRIFT
}

fn main() {
    print!("Give temperature in Celsius: ");
    stdout().flush().unwrap();
    let mut buff = String::new();
    let res = stdin().read_line(&mut buff);
    if let Err(e) = res {
        err(Box::new(e), "while reading temperature ");
    }
    buff = buff.trim().to_owned();
    let faren_temp = buff.parse::<f64>();
    match faren_temp {
        Err(e) => err(Box::new(e), "temperature must be a number, "),
        Ok(x) => {
            let faren_temp = cels_to_faren(x);
            println!("{}℃ in farenheits {}℉", x, faren_temp);
        }
    }
}
#[cfg(test)]
mod tests;
