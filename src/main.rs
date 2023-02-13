use clap::Parser;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, Error, Read},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Operator {
    Increase = b'+',  // 43
    Decrease = b'-',  // 45
    MoveRight = b'>', // 62
    MoveLeft = b'<',  // 60
    Output = b'.',    // 46
    Input = b',',     // 44
    JumpRight = b'[', // 91
    JumpLeft = b']',  // 93
}

impl TryFrom<u8> for Operator {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'+' => Ok(Operator::Increase),
            b'-' => Ok(Operator::Decrease),
            b'>' => Ok(Operator::MoveRight),
            b'<' => Ok(Operator::MoveLeft),
            b'.' => Ok(Operator::Output),
            b',' => Ok(Operator::Input),
            b'[' => Ok(Operator::JumpRight),
            b']' => Ok(Operator::JumpLeft),
            _ => Err("unknown operator!"),
        }
    }
}

struct Interpreter {
    source: Vec<u8>,
    memory: Vec<u8>,
    pointer: u64,
    pc: u64,
    jump_map: HashMap<u64, u64>,
}

impl Interpreter {
    fn new(source: Vec<u8>) -> Self {
        Interpreter {
            source,
            memory: vec![0; 30000],
            pointer: 0_u64,
            pc: 0_u64,
            jump_map: HashMap::<u64, u64>::new(),
        }
    }

    fn record_condition_location(&mut self) {
        let mut v = Vec::new();
        for i in 0..self.source.len() {
            match self.source[i].try_into().unwrap() {
                Operator::JumpRight => v.push(i),
                Operator::JumpLeft => {
                    let x = v.pop().expect("invalid program");
                    self.jump_map.insert(x as u64, i as u64);
                    self.jump_map.insert(i as u64, x as u64);
                }
                _ => (),
            }
        }
    }

    fn run(&mut self) {
        self.record_condition_location();

        while self.pc < self.source.len() as u64 {
            match self.source[self.pc as usize].try_into().unwrap() {
                Operator::Increase => {
                    self.memory[self.pointer as usize] =
                        self.memory[self.pointer as usize].wrapping_add(1);
                }
                Operator::Decrease => {
                    self.memory[self.pointer as usize] -= 1;
                }
                Operator::MoveRight => {
                    self.pointer += 1;
                    self.pointer %= self.memory.len() as u64;
                }
                Operator::MoveLeft => {
                    self.pointer -= 1;
                    // self.pointer += self.memory.len() as u64;
                    self.pointer %= self.memory.len() as u64;
                }
                Operator::Output => {
                    print!("{}", self.memory[self.pointer as usize] as char);
                }
                Operator::Input => {
                    let mut s = String::new();
                    io::stdin().read_line(&mut s).unwrap();
                    self.memory[self.pointer as usize] = s.as_bytes()[0];
                }
                Operator::JumpRight => {
                    if self.memory[self.pointer as usize] == 0 {
                        let x = self.jump_map.get(&self.pc).expect("");
                        self.pc = *x;
                    }
                }
                Operator::JumpLeft => {
                    if self.memory[self.pointer as usize] != 0 {
                        let x = self.jump_map.get(&self.pc).expect("Error");
                        self.pc = *x;
                    }
                }
            }

            self.pc += 1;
        }
    }
}

/// brainfuck interpreter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the brainfuck file to interpreter
    #[arg(short, long)]
    run: Option<String>,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let path = args.run;

    match path {
        Some(path) => {
            let mut file = File::open(path)?;
            let mut text = String::new();
            file.read_to_string(&mut text)?;

            let mut source = text.as_bytes().to_vec();
            source.retain(
                |&x| <u8 as TryInto<Operator>>::try_into(x).is_ok()
            );

            let mut interpreter = Interpreter::new(source);
            interpreter.run();
        },
        None => println!("Error"),
    }

    Ok(())
}
