use std::{error::Error, str::FromStr};

use thiserror::Error;

use crate::{ints::{Word11, Word12}, pinboards::{Instruction, Ones, Opcode, Pinboard, Tens}, io::IO};

// TODO: Sound the alarm instead of erroring?
#[derive(Debug, Error)]
enum ExecutionError<IOError> {
    #[error("Missing pinboard {0}")]
    MissingPinboard(u8),
    #[error("Missing instruction. Pinboard {0} instruction {1}")]
    MissingInstruction(u8, u8),
    #[error("Invalid memory reference: {0} {1}")]
    InvalidMemory(u8, u8),
    #[error("B Transfer too large")]
    BTooLarge,
    #[error("Overflow")]
    Overflow,
    #[error("IO Error: {0}")]
    IOError(#[from] IOError)
}



enum Status {
    Halt,
    Keyboard,
    Alarm,
    Continue
}

struct Memory([Word12; 100]);

impl Memory {
    fn get<IO: crate::io::IO>(&mut self, tens: u8, ones: u8) -> Result<&mut Word12, ExecutionError<IO::Error>> {
        if 10 <= tens || 10 <= ones {
            return Err(ExecutionError::InvalidMemory(tens, ones))
        }
        Ok(&mut self.0[(10 * tens + ones) as usize])
    }
}



struct Cpu {
    status: Status,
    pinboards: [Option<Pinboard>; 8], // Remember! Instructions index pinboards from 1!,
    current_pinboard: u8,
    memory: Memory,
    a: Word12,
    b: Word11,
    e: u8,
    f: u8,
    x: u8,
    y: u8
}

impl Cpu {
    fn process_special_digits(&mut self, instruction: &mut Instruction) {
        let mut tens = instruction.1;
        let mut ones = instruction.2;
        if matches!(tens, Some(Tens::E)) {
            tens = Some(Tens::Num(self.e));
        }
        if matches!(ones, Some(Ones::E)) {
            ones = Some(Ones::Num(self.e));
        }
        if matches!(ones, Some(Ones::F)) {
            ones = Some(Ones::Num(self.f));
        }
        if matches!(tens, Some(Tens::X)) {
            tens = Some(Tens::Num(self.x));
        }
        if matches!(ones, Some(Ones::Y)) {
            ones = Some(Ones::Num(self.y));
        }
        *instruction = Instruction(instruction.0, tens, ones);
    }
    fn step<IO: crate::io::IO>(&mut self, io: &mut IO) -> Result<(), ExecutionError<IO::Error>> {
        let pinboard = self.pinboards[self.current_pinboard as usize].as_ref().ok_or(ExecutionError::MissingPinboard(self.current_pinboard))?;
        let instruction = pinboard.instructions[pinboard.next_instruction as usize]
                                                               .as_ref().ok_or(ExecutionError::MissingInstruction(self.current_pinboard, pinboard.next_instruction))?;
        let mut instruction = *instruction;
        self.process_special_digits(&mut instruction);
        
        match instruction {
            Instruction(Opcode::K, _, _) => {
                // TODO?: Non-printing keyboard
                self.status = Status::Keyboard;
                // TODO: Check math for negatives
                self.a = io.keyboard()? % 100_000_000_000;
            },
            Instruction(Opcode::W, Some(Tens::Num(tens)), Some(Ones::Num(ones))) => {
                *self.memory.get::<IO>(tens, ones)? = self.a;
            },
            Instruction(Opcode::R, Some(Tens::Num(tens)), Some(Ones::Num(ones))) => {
                self.a = *self.memory.get::<IO>(tens, ones)?;
            },
            Instruction(Opcode::B, _, _) => {
                self.b = Word11::new(self.a.get()).ok_or(ExecutionError::BTooLarge)?;
            }
            Instruction(Opcode::P, _, Some(Ones::Num(0))) => {
                // No-op
                // Originally for carriage control, but I'm not supporting that.
            },
            Instruction(Opcode::P, _, Some(Ones::Star)) => {
                io.print(self.a)?;
                self.status = Status::Halt;
            },
            Instruction(Opcode::P, _, _) => {
                io.print(self.a)?;
            },
            Instruction(Opcode::Plus, Some(Tens::Num(tens)), Some(Ones::Num(ones))) => {
                self.a = self.a.checked_add(self.memory.get::<IO>(tens, ones)?.get()).ok_or(ExecutionError::Overflow)?;
                // Currently not implemented: Proper continuation after alarm
            }
            Instruction(Opcode::Minus, Some(Tens::Num(tens)), Some(Ones::Num(ones))) => {
                self.a = self.a.checked_sub(self.memory.get::<IO>(tens, ones)?.get()).ok_or(ExecutionError::Overflow)?;
                // Currently not implemented: Proper continuation after alarm
            },
            Instruction(Opcode::Mult, Some(Tens::Num(tens)), Some(Ones::Num(ones))) => {
                let multiplied = self.a.get() * self.b.get();
                let significant = multiplied / 1_00_000_000_000;
                self.a = Word12::new(significant).ok_or(ExecutionError::Overflow)?;
            }
            
            
        }

        Ok(())
    }
}