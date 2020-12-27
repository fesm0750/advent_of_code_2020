use crate::helpers::read;

use std::{cmp::Ordering, error::Error, str::FromStr};

//--------------------------------------------------------------------
// Console Data Structures
//--------------------------------------------------------------------

#[derive(Default)]
struct ConsoleDebugger {
    instr_list: Vec<DebugInstruction>,
    backtrace: Vec<usize>,
    state: State,
}

//------------------------------
// Instructions
//------------------------------

struct DebugInstruction {
    instr: Instruction,
    visited: bool,
}

type Instruction = (Op, i16);

enum Op {
    Acc, // Accumulate
    Jmp, // Jump
    Nop, // No Operation
}

//------------------------------
// Internal State
//------------------------------

#[derive(Default, Copy, Clone)]
struct State {
    pc: usize, // program counter
    acc: i16,  // accumulator
    status: Status,
}

#[derive(Copy, Clone)]
enum Status {
    Crash,   // Negative program counter state
    InfLoop, // Infinite loop
    OutOfBounds,
    SiInfLoop, // Single Instruction Infinite Loop
    Running,
    Success,
}

//--------------------------------------------------------------------
// Solution
//--------------------------------------------------------------------

/// # Assumption
///
/// Input data is well behaved.
fn parse_input(input: &str) -> ConsoleDebugger {
    let instr_list = read::lines_into_vec(input);
    ConsoleDebugger::new(instr_list)
}

pub fn run() {
    println!("Day 08");
    let input = read::to_str("day08").unwrap();
    let mut console = parse_input(&input);
    let state = console.debug();
    println!(
        "Value of the accumulator before infinite loop: {}",
        state.acc
    );
    let right_acc = console.fix();
    println!("Correct accumulator value: {}", right_acc);
}

//--------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------

impl ConsoleDebugger {
    fn new(instr_list: Vec<DebugInstruction>) -> Self {
        Self {
            instr_list,
            ..Self::default()
        }
    }

    // returns the state of the console at the moment of a successuful completion,
    // crash or detected infinity loop.
    //
    // The program is supposed to terminate by attempting to execute an instruction
    // immediately after the last instruction in the file
    fn debug(&mut self) -> State {
        loop {
            let pc = self.state.pc;

            // Success
            if pc == self.instr_list.len() {
                self.state.status = Status::Success;
            }

            // Error cases
            if pc > self.instr_list.len() {
                self.state.status = Status::OutOfBounds;
            }

            // Returns if an error has happened,
            // also catches status tha may have happened inside run_instruction
            //? is there any way to use a negation if let?
            //? if let !State::Runnig = state {}
            match self.state.status {
                Status::Running => (),
                _ => return self.state,
            }

            if self.instr_list[pc].visited {
                self.state.status = Status::InfLoop;
                return self.state;
            }

            self.instr_list[pc].visited = true;
            self.backtrace.push(pc);

            self.run_instruction();
        }
    }

    // fix the console and returns the final value of the accumulator
    fn fix(&mut self) -> i16 {
        let mut saved_state = self.state;
        let mut first = true;
        loop {
            #[allow(clippy::single_match)]
            match self.state.status {
                Status::Success => return self.state.acc,
                // Status::InfLoop | Status::OutOfBounds => {
                //     self.state.pc = *self.backtrace.last().unwrap()
                // }
                _ => (),
            }

            // rewinds
            // attempts are fully rewinded
            // original execution (og) is rewinded until next jmp or nop
            let mut reached_og = false;
            while let Some(&ip) = self.backtrace.last() {
                if ip == saved_state.pc {
                    reached_og = true;
                }
                if let (Op::Acc, val) = self.instr_list[ip].instr {
                    self.state.acc -= val;
                } else if (reached_og && ip != saved_state.pc) || first {
                    // reset running state to this point
                    self.state.pc = ip;
                    self.state.status = Status::Running;
                    break;
                }

                self.instr_list[ip].visited = false;
                self.backtrace.pop();
            }

            // Problem cannot be fixed.
            if self.backtrace.is_empty() {
                panic!("Unable to fix console");
            }

            // saves state of the program at current pc
            saved_state = self.state;

            // executes a changed instruction trying to fix code
            // nop becomes jmp, and all other instruction advance one
            if let (Op::Nop, val) = self.instr_list[self.state.pc].instr {
                self.jump(val);
            } else {
                self.jump(1);
            }

            self.debug();
            first = false;
        }
    }

    // Runs the the current instruction. Returns `false` if the execution leads
    // to a corrupted state.
    fn run_instruction(&mut self) {
        let jmp;
        match self.instr_list[self.state.pc].instr {
            (Op::Acc, val) => {
                self.state.acc += val;
                jmp = 1
            }
            (Op::Jmp, val) => jmp = val,
            _ => jmp = 1,
        }
        self.jump(jmp);
    }

    /// Tries to jump the program counter by n_jmps.
    /// Changes internal state status if jump could not be done.
    fn jump(&mut self, n_jmps: i16) {
        match n_jmps.cmp(&0i16) {
            Ordering::Greater => self.state.pc += n_jmps.abs() as usize,
            Ordering::Less => {
                let jmp = n_jmps.abs() as usize;
                if jmp > self.state.pc {
                    self.state.status = Status::Crash;
                } else {
                    self.state.pc -= jmp;
                }
            }
            Ordering::Equal => self.state.status = Status::SiInfLoop,
        }
    }
}

impl DebugInstruction {
    pub fn new(instr: Instruction) -> Self {
        Self {
            instr,
            visited: false,
        }
    }
}

impl FromStr for DebugInstruction {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();
        let op = iter.next().ok_or("Could not get Instruction code.")?;
        let op = match op {
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            "nop" => Op::Nop,
            _ => panic!("Wrong Instruction code in input."),
        };
        let arg: i16 = iter.next().unwrap().parse()?;
        Ok(DebugInstruction::new((op, arg)))
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Running
    }
}

//--------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn tests() {
        let mut console = parse_input(INPUT);
        assert_eq!(console.debug().acc, 5);
        let acc = console.fix();
        assert_eq!(acc, 8);
    }
}
