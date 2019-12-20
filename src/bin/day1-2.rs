use std::cmp;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();
    let mut total_fuel = 0;
    while let Some(Ok(line)) = iter.next() {
        let mut mass = line.parse::<i32>().unwrap();
        while mass > 0 {
            let fuel = ((mass as f32 / 3.0).floor() - 2.0) as i32;
            let fuel = cmp::max(fuel, 0);
            total_fuel += fuel;
            mass = fuel;
            println!("{}", mass);
        }
    }
    println!("{}", total_fuel);
}
