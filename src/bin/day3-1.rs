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

pub fn find_crossed_wires(red: &[Dir], blue: &[Dir]) -> Option<isize> {
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
    let best_crossed = all_red
        .intersection(&all_blue)
        .map(|&(x, y)| x.abs() + y.abs())
        .min();
    best_crossed
}

fn main() {
    let red_wire = parse_wire_dirs();
    let blue_wire = parse_wire_dirs();
    let dist = find_crossed_wires(&red_wire, &blue_wire).unwrap();
    println!("{}", dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dir_test() {
        let dir_str = "R8,U5,L5,D3";
    }

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
}
