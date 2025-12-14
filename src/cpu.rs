use std::str::FromStr;

use thiserror::Error;

use crate::{ints::{Word11, Word12}, pinboards::{Instruction, Ones, Opcode, Pinboard, Tens}};

// TODO: Sound the alarm instead of erroring?
#[derive(Debug, Error)]
enum ExecutionError {
    #[error("Missing pinboard {0}")]
    MissingPinboard(u8),
    #[error("Missing instruction. Pinboard {0} instruction {1}")]
    MissingInstruction(u8, u8),
    #[error("Invalid memory reference: {0} {1}")]
    InvalidMemory(u8, u8)
}



enum Status {
    Halt,
    Keyboard,
    Alarm,
    Continue
}

struct Memory([Word12; 100]);

impl Memory {
    fn get(&mut self, tens: u8, ones: u8) -> Result<&mut Word12, ExecutionError> {
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
    fn step(&mut self) -> Result<(), ExecutionError> {
        let pinboard = self.pinboards[self.current_pinboard as usize].as_ref().ok_or(ExecutionError::MissingPinboard(self.current_pinboard))?;
        let instruction = pinboard.instructions[pinboard.next_instruction as usize]
                                                               .as_ref().ok_or(ExecutionError::MissingInstruction(self.current_pinboard, pinboard.next_instruction))?;
        let mut instruction = *instruction;
        self.process_special_digits(&mut instruction);
        
        match instruction {
            Instruction(Opcode::K, _, _) => {
                // TODO?: Non-printing keyboard
                self.status = Status::Keyboard;
                return Ok(());
            },
            Instruction(Opcode::W, Some(Tens::Num(tens)), Some(Ones::Num(ones))) => {
                *self.memory.get(tens, ones)? = self.a;
            }
        }

        Ok(())
    }
}