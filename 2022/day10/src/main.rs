use core::panic;
use core::fmt::Debug;
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{IndexMut, Index};
use std::path::Path;

///////// Opcode implementations

#[derive(Debug)]
enum OpcodeType {
    NOOP,
    ADDX,
}

#[derive(Debug)]
struct NoopOpcode {
    current_state: u32,
}

impl NoopOpcode {
    fn new() -> Self {
        NoopOpcode { current_state: 0, }
    }
}

impl Opcode for NoopOpcode {
    fn tick(&mut self) {
        if !self.done() {
            self.current_state += 1;
        }
    }

    fn done(&self)->bool {
        self.current_state >= 1
    }

    fn get_type(&self)->OpcodeType {
        OpcodeType::NOOP
    }

    fn apply(&self, _machine:&mut Registers) {
        // noop
    }
}

#[derive(Debug)]
struct AddxOpcode {
    current_state: u32,
    arg: i64,
}

impl AddxOpcode {
    fn new(arg_:i64) -> Self {
        AddxOpcode { current_state: 0, arg:arg_}
    }
}

impl Opcode for AddxOpcode {
    fn tick(&mut self) {
        if !self.done() {
            self.current_state += 1;
        }
    }

    fn done(&self)->bool {
        self.current_state >= 2
    }

    fn get_type(&self)->OpcodeType {
        OpcodeType::ADDX
    }

    fn apply(&self, registers:&mut Registers) {
       *registers.borrow_mut().index_mut(RegName::X) += self.arg;
    }
}

trait Opcode {
    fn tick(&mut self);
    fn done(&self)->bool;
    fn get_type(&self)->OpcodeType;
    fn apply(&self, machine:&mut Registers);
}

impl Debug for dyn Opcode {
    fn fmt(&self, f:&mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.get_type())
    }
}

///////// Register banks implementations

enum RegName {
    PC,
    X,
}

struct Registers {
    registers: Vec<i64>,
}

impl Registers {
    fn new(size: usize) -> Self {
        Registers {
            registers:vec![0; size]
        }
    }
}

impl Index<RegName> for Registers {
    type Output = i64;

    fn index(&self, index: RegName) -> &Self::Output {
        match index {
            RegName::PC => self.registers.index(0),
            RegName::X => self.registers.index(1),
        }
    }
}

impl IndexMut<RegName> for Registers {
    fn index_mut(&mut self, index: RegName) -> &mut Self::Output {
        match index {
            RegName::PC => self.registers.index_mut(0),
            RegName::X => self.registers.index_mut(1),
        }
    }
}

///////// Crt

struct Crt {
    screen : Vec<char>,
    clock:usize,
}

impl Crt {
    fn new() -> Self {
        Crt {
            screen : vec!['·'; 40 * 6],
            clock: 0,
        }
    }

    fn clock(&mut self, regs:&Registers) {
        let middle = regs.index(RegName::X).to_owned();
        let r = (middle - 1)..=(middle + 1);
        let hpos = (self.clock % 40) as i64;

        if r.contains(&hpos)  {
            self.screen[self.clock] = '█';
        }
        println!("  CRT at {} (X={})", self.clock, middle);
        self.clock += 1;

        if self.clock > (40 * 6) {
            self.clock = 0;
        }
    }

    fn render(&self) {
        for y in 0..6 {
            for x in 0..40 {
                print!("{}", self.screen[y * 40 + x]);
            }
            println!();
        }
    }
}


///////// Machine implementations

struct Machine {
    program: Vec<Box<dyn Opcode>>,
    registers: Registers,
    screen: Crt,
    clock: i64,
}

impl Machine {
    fn new() -> Self {
        Machine {
            program: Vec::new(),
            registers: Registers::new(3),
            screen: Crt::new(),
            clock:0,
        }
    }

    fn run(&mut self) {
        *self.registers.index_mut(RegName::X) = 1;
        let mut sum = 0;
        loop {
            let ip:usize = *self.registers.index(RegName::PC) as usize;
            self.program.index_mut(ip).tick();
            let op = self.program.index(ip).clone();

            println!("cycle: {}", self.clock + 1);
            self.screen.clock(&self.registers.borrow_mut());

            //println!("Fetched Opcode {:?} from pc {}", op, ip);
            if vec![19i64, 59i64, 99i64, 139i64, 179i64, 219i64].contains(&self.clock) {
                // println!("X is {}", self.registers.index(RegName::X));
                sum += (self.clock + 1) * self.registers.index(RegName::X);
            }
            if op.done() {
                op.apply(self.registers.borrow_mut());
                *self.registers.index_mut(RegName::PC) += 1;
                if *self.registers.index_mut(RegName::PC) as usize >= self.program.len() {
                    break;
                }
            }
            self.clock += 1;
        }

        println!("Sum of signals: {}", sum);
        self.screen.render();
    }
}

fn main() {
    // File hosts must exist in current path before this produces output
    let Ok(lines) = read_lines("input") else { return };

    let mut machine = Machine::new();

    for line in lines {
        if let Ok(ip) = line {
            let mut code = ip.split(" ");
            match code.next() {
                Some("noop") => {
                    machine.program.push(Box::new(NoopOpcode::new()));
                },
                Some("addx") => {
                    let Some(value) = code.next() else { panic!("Invalid program")};
                    let arg = value.parse::<i64>().unwrap();
                    machine.program.push(Box::new(AddxOpcode::new(arg)))
                },
                _ => { panic!("Invalid program"); }
            }
        }
    }

    machine.run();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}