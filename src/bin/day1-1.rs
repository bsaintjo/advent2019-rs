use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let mut total_fuel = 0.0;
    while let Some(Ok(line)) = iter.next() {
        let mass = line.parse::<i32>().unwrap();
        let fuel = (mass as f32 / 3.0).floor() - 2.0;
        total_fuel += fuel;
    }
    println!("{}", total_fuel);
}
