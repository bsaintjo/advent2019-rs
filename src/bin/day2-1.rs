use itertools::join;
use std::io::stdin;

fn run_program(prog: &mut [usize]) {
    let mut iter = 0..prog.len();
    loop {
        // let maybe_idx = iter.next();
        while let Some(idx) = iter.next() {
            match prog[idx] {
                1 => {
                    let fst_val = prog[prog[idx + 1]];
                    let snd_val = prog[prog[idx + 2]];
                    prog[prog[idx + 3]] = fst_val + snd_val;
                    iter.next();
                    iter.next();
                    iter.next();
                }
                2 => {
                    let fst_val = prog[prog[idx + 1]];
                    let snd_val = prog[prog[idx + 2]];
                    prog[prog[idx + 3]] = fst_val * snd_val;
                    iter.next();
                    iter.next();
                    iter.next();
                }
                99 => {
                    break;
                }
                _ => {
                    todo!();
                }
            }
        }
        break;
    }
}

fn main() {
    let mut program = String::new();
    let _ = stdin().read_line(&mut program);
    program.pop();
    let mut program_ops: Vec<usize> = program.split(',').map(|x| x.parse().unwrap()).collect();
    run_program(&mut program_ops);
    let ran_program = join(program_ops, ",");
    println!("{}", ran_program);
}
