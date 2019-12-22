use std::collections::HashSet;
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
    let program_ops: Vec<usize> = program.split(',').map(|x| x.parse().unwrap()).collect();
    let mut tried: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..=99 {
        for j in 0..=99 {
            if tried.contains(&(j, i)) {
                continue;
            }
            tried.insert((j, i));
            let mut program_copy = program_ops.clone();
            program_copy[1] = i;
            program_copy[2] = j;
            run_program(&mut program_copy);
            if program_copy[0] == 19690720 {
                println!(
                    "noun={}, verb={}, 100 * verb + noun = {}",
                    i,
                    j,
                    100 * i + j
                );
            }
        }
    }
}
