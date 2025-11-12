use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
enum Opcode {
    K, // Keyboard
    W, // Write
    R, // Read
    B, // Accum -> B
    P, // Print
    Plus, // +
    Minus, // -
    Mult, // ×
    Div, // ÷
    A, // Alter.
    U, // Unconditional transfer,
    C, // Conditional transfer,
    H, // Home
    S, // Step
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Unknown opcode: {0}")]
    UnknownOpcode(String),
    #[error("Unknown tens: {0}")]
    UnknownTens(String),
    #[error("Unknown ones: {0}")]
    UnknownOnes(String),
    #[error("Unable to parse instruction: {0}")]
    ParseInstructionFailure(String)
}

impl FromStr for Opcode {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "K" => Ok(Opcode::K),
            "W" => Ok(Opcode::W),
            "R" => Ok(Opcode::R),
            "B" => Ok(Opcode::B),
            "P" => Ok(Opcode::P),
            "+" => Ok(Opcode::Plus),
            "-" => Ok(Opcode::Minus),
            "×" => Ok(Opcode::Mult),
            "÷" => Ok(Opcode::Div),
            "A" => Ok(Opcode::A),
            "U" => Ok(Opcode::U),
            "C" => Ok(Opcode::C),
            "H" => Ok(Opcode::H),
            "S" => Ok(Opcode::S),
            _ => Err(ParseError::UnknownOpcode(String::from(s)))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tens {
    Num(u8),
    E,
    X
}

impl FromStr for Tens {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "E" => Ok(Tens::E),
            "X" => Ok(Tens::X),
            _ => {
                str::parse::<u8>(s)
                    .map_err(|_| ParseError::UnknownTens(String::from(s)))
                    .and_then(|num| {
                        if num <= 9 {
                            Ok(Tens::Num(num))
                        } else {
                            Err(ParseError::UnknownTens(String::from(s)))
                        }
                    })
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Ones {
    Num(u8),
    E,
    F,
    Y,
    Star,
    V
}

impl FromStr for Ones {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "E" => Ok(Ones::E),
            "F" => Ok(Ones::F),
            "Y" => Ok(Ones::Y),
            "*" => Ok(Ones::Star),
            "V" => Ok(Ones::V),
            _ => {
                str::parse::<u8>(s)
                    .map_err(|_| ParseError::UnknownOnes(String::from(s)))
                    .and_then(|num| {
                        if num <= 15 {
                            Ok(Ones::Num(num))
                        } else {
                            Err(ParseError::UnknownOnes(String::from(s)))
                        }
                    })
            }
        }
    }
}

#[derive(Debug)]
struct Instruction(Opcode, Option<Tens>, Option<Ones>);

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.is_empty() {
            return Err(ParseError::ParseInstructionFailure(String::from(s)))
        }
        let opcode: Opcode = str::parse(parts.get(0).ok_or_else(|| ParseError::ParseInstructionFailure(String::from(s)))?)?;
        let tens: Option<Tens> = parts.get(1).map(|s| s.parse()).transpose()?;
        let ones: Option<Ones> = parts.get(2).map(|s| s.parse()).transpose()?;
        Ok(Instruction(opcode, tens, ones))
    }
}