use std::collections::HashSet;
use std::io::stdin;
use std::iter;

#[derive(PartialEq, Debug)]
pub enum Dir {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

impl Dir {
    pub fn from_str(wire_seg: &str) -> Self {
        let (dir, size) = wire_seg.split_at(1);
        let size: isize = size.parse().unwrap();
        match dir {
            "U" => Dir::Up(size),
            "D" => Dir::Down(size),
            "L" => Dir::Left(size),
            "R" => Dir::Right(size),
            _ => todo!(),
        }
    }

    pub fn to_positions(&self, x: &mut isize, y: &mut isize) -> Vec<(isize, isize)> {
        let poss = match &self {
            Dir::Up(size) => {
                let xs = iter::repeat(*x).zip(*y..*y + size + 1).collect();
                *y += size;
                xs
            }
            Dir::Down(size) => {
                let xs = iter::repeat(*x).zip(*y - size..*y - 1).collect();
                *y -= size;
                xs
            }
            Dir::Left(size) => {
                let xs = (*x - size..*x - 1).zip(iter::repeat(*y)).collect();
                *x -= size;
                xs
            }
            Dir::Right(size) => {
                let xs = (*x..*x + size + 1).zip(iter::repeat(*y)).collect();
                *x += size;
                xs
            }
        };
        poss
    }
}

pub fn parse_wire_dirs() -> Vec<Dir> {
    let mut wire_str = String::new();
    let _ = stdin().read_line(&mut wire_str);
    wire_str.pop();
    wire_str.split(',').map(|x| Dir::from_str(x)).collect()
}

pub fn is_between(start: &(isize, isize), end: &(isize, isize), point: &(isize, isize)) -> bool {
    if start.0 == point.0 {
        let (lt, gt) = if end.1 > start.1 {
            (start.1, end.1)
        } else {
            (end.1, start.1)
        };
        if lt <= point.1 && point.1 <= gt {
            return true;
        }
    } else if start.1 == point.1 {
        let (lt, gt) = if end.0 > start.0 {
            (start.0, end.0)
        } else {
            (end.0, start.0)
        };
        if lt <= point.0 && point.0 <= gt {
            return true;
        }
    }
    false
}

pub fn trace(wire: &[Dir], cross: &(isize, isize)) -> usize {
    let mut acc = 0;
    let mut x = 0;
    let mut y = 0;
    for dir in wire.iter() {
        let last_x = x;
        let last_y = y;
        let _ = dir.to_positions(&mut x, &mut y);
        if is_between(&(x, y), &(last_x, last_y), cross) {
            acc += (last_x - cross.0).abs() + (last_y - cross.1);
            break;
        } else {
            acc += (x - last_x).abs() + (y - last_y).abs();
        }
    }
    acc as usize
}

pub fn find_crossed_wires(red: &[Dir], blue: &[Dir]) -> Vec<(isize, isize)> {
    let mut x = 0;
    let mut y = 0;
    let all_red: HashSet<(isize, isize)> = red
        .iter()
        .map(|d| d.to_positions(&mut x, &mut y))
        .flatten()
        .skip(1) // Remove (0, 0)
        .collect();
    x = 0;
    y = 0;
    let all_blue: HashSet<(isize, isize)> = blue
        .iter()
        .map(|d| d.to_positions(&mut x, &mut y))
        .flatten()
        .skip(1) // Remove (0, 0)
        .collect();
    all_red.intersection(&all_blue).map(|&a| a).collect()
}

fn main() {
    let red_wire = parse_wire_dirs();
    let blue_wire = parse_wire_dirs();
    let crosses = find_crossed_wires(&red_wire, &blue_wire);
    let closest_traced_cross = crosses
        .iter()
        .map(|cross| trace(&red_wire, cross) + trace(&blue_wire, cross))
        .min()
        .unwrap();
    println!("{}", closest_traced_cross);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_test() {
        let mut x = 0;
        let mut y = 0;
        let dir1 = Dir::from_str("L32");
        let dir2 = Dir::from_str("U7");

        assert_eq!(dir1, Dir::Left(32));
        assert_eq!(dir2, Dir::Up(7));

        let _ = dir1.to_positions(&mut x, &mut y);
        assert_eq!(x, -32);
        assert_eq!(y, 0);

        let _ = dir2.to_positions(&mut x, &mut y);
        assert_eq!(x, -32);
        assert_eq!(y, 7);
    }

    #[test]
    fn is_between_test() {
        let start = (0, 0);
        let point_a = (0, 6);
        let end_a = (0, 9);

        let point_b = (3, 9);
        let end_b = (6, 9);

        let outside = (5, 5);

        // Normal
        assert!(is_between(&start, &end_a, &point_a));
        assert!(is_between(&end_a, &end_b, &point_b));

        // Opposite direction
        assert!(is_between(&end_a, &start, &point_a));
        assert!(is_between(&end_b, &end_a, &point_b));

        // Outside
        assert!(!is_between(&start, &end_a, &outside));
        assert!(!is_between(&end_a, &end_b, &outside));
    }
}
