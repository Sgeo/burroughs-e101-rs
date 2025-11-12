use std::str::FromStr;

use thiserror::Error;

use crate::pinboards::Pinboard;



enum Status {
    Halt,
    Keyboard,
    Continue
}

struct Memory([[i64; 10]; 10]);



struct Cpu {
    status: Status,
    pinboards: [Option<Pinboard>; 8], // Remember! Instructions index pinboards from 1!
    current_pinboard: u8,
    a: i64,
    b: i64,
    e: u8,
    f: u8,
    x: u8,
    y: u8
}