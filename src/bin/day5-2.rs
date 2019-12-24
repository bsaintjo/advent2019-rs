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
    JumpIfTrue(Mode, Mode, Mode),
    JumpIfFalse(Mode, Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equals(Mode, Mode, Mode),
    Save,
    Output,
    Halt,
}

impl OpCode {
    pub fn parse(x: isize) -> Self {
        let op = x % 10;
        let (fst, snd, thrd) = OpCode::parse_params(x);
        match op {
            1 => {
                OpCode::Add(fst, snd, thrd)
            },
            2 => {
                OpCode::Multiply(fst, snd, thrd)
            },
            3 => OpCode::Save,
            4 => OpCode::Output,
            5 => OpCode::JumpIfTrue(fst, snd, thrd),
            6 => OpCode::JumpIfFalse(fst, snd, thrd),
            7 => OpCode::LessThan(fst, snd, thrd),
            8 => OpCode::Equals(fst, snd, thrd),
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
        println!("{}", inst);
        println!("{:?}", opcode);
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
            OpCode::JumpIfTrue(fst_mode, snd_mode, _) => {
                let fst_addr = program[ip + 1];
                let snd_addr = program[ip + 2];

                let fst_val = fetch(fst_addr, fst_mode, &program);
                let snd_val = fetch(snd_addr, snd_mode, &program);
                if fst_val != 0 {
                    ip = snd_val as usize;
                } else {
                    ip += 3;
                }
            },
            OpCode::JumpIfFalse(fst_mode, snd_mode, _) => {
                let fst_addr = program[ip + 1];
                let snd_addr = program[ip + 2];

                let fst_val = fetch(fst_addr, fst_mode, &program);
                let snd_val = fetch(snd_addr, snd_mode, &program);
                if fst_val == 0 {
                    ip = snd_val as usize;
                } else {
                    ip += 3;
                }
            },
            OpCode::LessThan(fst_mode, snd_mode, _) => {
                let fst_addr = program[ip + 1];
                let snd_addr = program[ip + 2];
                let thrd_addr = program[ip + 3];

                let fst_val = fetch(fst_addr, fst_mode, &program);
                let snd_val = fetch(snd_addr, snd_mode, &program);

                if fst_val < snd_val {
                    program[thrd_addr as usize] = 1;
                } else {
                    program[thrd_addr as usize] = 0;
                }
                ip += 4;
            }
            OpCode::Equals(fst_mode, snd_mode, _) => {
                let fst_addr = program[ip + 1];
                let snd_addr = program[ip + 2];
                let thrd_addr = program[ip + 3];

                let fst_val = fetch(fst_addr, fst_mode, &program);
                let snd_val = fetch(snd_addr, snd_mode, &program);

                if fst_val == snd_val {
                    program[thrd_addr as usize] = 1;
                } else {
                    program[thrd_addr as usize] = 0;
                }
                ip += 4;
            }
            OpCode::Halt => break,
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut program = String::new();
    stdin().read_line(&mut program)?;
    let program: Vec<isize> = program.trim().split(',').map(|x| x.parse().unwrap()).collect();
    run(program);
    Ok(())
}
