use std::io::stdin;
use std::env;

#[derive(Debug)]
pub enum Mode {
    Position,
    Immediate,
}

impl Mode {
    pub fn new(x: isize) -> Self {
        if x == 0 {
            Mode::Position
        } else if x == 1 {
            Mode::Immediate
        } else {
            todo!()
        }
    }
}

#[derive(Debug)]
pub enum OpCode {
    Multiply(Mode, Mode, Mode),
    Add(Mode, Mode, Mode),
    Save,
    Output,
    Halt,
}

impl OpCode {
    pub fn parse(x: isize) -> Self {
        let op = x % 10;
        match op {
            1 => {
                let (fst, snd, thrd) = OpCode::parse_params(x);
                OpCode::Add(fst, snd, thrd)
            },
            2 => {
                let (fst, snd, thrd) = OpCode::parse_params(x);
                OpCode::Multiply(fst, snd, thrd)
            },
            3 => OpCode::Save,
            4 => OpCode::Output,
            99 => OpCode::Halt,
            _ => todo!(),
        }
    }

    pub fn parse_params(x: isize) -> (Mode, Mode, Mode) {
        let fst_mode = Mode::new((x / 100) % 10);
        let snd_mode = Mode::new((x / 1_000) % 10);
        let thrd_mode = Mode::new((x / 10_000) % 10);
        (fst_mode, snd_mode, thrd_mode)
    }

}

pub fn fetch(position: isize, mode: Mode, program: &[isize]) -> isize {
    match mode {
        Mode::Immediate => { position as isize},
        Mode::Position => { program[position as usize] },
    }
}

fn run(mut program: Vec<isize>) {
    let mut ip = 0;
    loop {
        let inst = program[ip];
        let opcode = OpCode::parse(inst);
        match opcode {
            OpCode::Multiply(fst_mode, snd_mode, _) => {
                let fst_addr = program[ip + 1];
                let snd_addr = program[ip + 2];
                let thrd_addr = program[ip + 3];

                let fst_val = fetch(fst_addr, fst_mode, &program);
                let snd_val = fetch(snd_addr, snd_mode, &program);
                program[thrd_addr as usize] = fst_val * snd_val;
                ip += 4;
            }
            OpCode::Add(fst_mode, snd_mode, _) => {
                let fst_addr = program[ip + 1];
                let snd_addr = program[ip + 2];
                let thrd_addr = program[ip + 3];

                let fst_val = fetch(fst_addr, fst_mode, &program);
                let snd_val = fetch(snd_addr, snd_mode, &program);
                program[thrd_addr as usize] = fst_val + snd_val;
                ip += 4;
            }
            OpCode::Output => {
                let addr = program[ip + 1];
                println!("{}", program[addr as usize]);
                ip += 2;
            },
            OpCode::Save => {
                let addr = program[ip + 1];
                let val = env::args().nth(1).expect("Pass on cmd-line").parse::<isize>().expect("An integer");
                program[addr as usize] = val;
                ip += 2;

            },
            OpCode::Halt => break,
        }
    }
}

fn main() {
    let mut program = String::new();
    let _ = stdin().read_line(&mut program);
    program.pop();
    let program: Vec<isize> = program.split(',').map(|x| x.parse().unwrap()).collect();
    run(program);
}
