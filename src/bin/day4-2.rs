use std::io::stdin;

fn check_num(n: usize) -> bool {
    let mut is_adjacent = false;
    let always_increasing = (1..=6)
        .rev()
        .map(|x| { let factor = 10usize.pow(x); (n % factor)  / (factor / 10) })
        .collect::<Vec<_>>()
        .windows(2)
        .all(|pair| {
            let x = pair[0];
            let y = pair[1];
            if y >= x {
                if y == x {
                    is_adjacent = true;
                }
                true
            } else {
                false
            }
        });
    always_increasing && is_adjacent
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
        assert!(check_num(111_111));
        assert!(check_num(111_123));
        assert!(check_num(135_679));
        assert!(!check_num(223_450));
        assert!(!check_num(123_789));
    }
}
