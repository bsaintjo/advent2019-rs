use std::collections::HashMap;
use std::io::stdin;

fn check_num(n: usize) -> bool {
    let mut is_adjacent = HashMap::new();
    let always_increasing = (1..=6)
        .rev()
        .map(|x| {
            let factor = 10usize.pow(x);
            (n % factor) / (factor / 10)
        })
        .collect::<Vec<_>>()
        .windows(2)
        .all(|pair| {
            let x = pair[0];
            let y = pair[1];
            if y >= x {
                if y == x {
                    let group_size = is_adjacent.entry(x).or_insert(1);
                    *group_size += 1;
                }
                true
            } else {
                false
            }
        });
    always_increasing && (is_adjacent.values().any(|&v| v == 2))
}

fn main() {
    let mut line = String::new();
    let _ = stdin().read_line(&mut line);
    line.pop();
    let nums: Vec<usize> = line.split('-').map(|x| x.parse().unwrap()).collect();
    let mut count = 0;
    for n in nums[0]..nums[1] {
        if check_num(n) {
            count += 1;
        }
    }
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_num_test() {
        assert!(check_num(112_233));
        assert!(!check_num(123_444));
        assert!(check_num(111_122));
    }
}
