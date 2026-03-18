#![deny(clippy::pedantic)]

#[cfg(not(target_pointer_width = "64"))]
compile_error!("it's an x64 simulator sorry");

mod instruction;
mod label_parser;
mod lexer;
mod operands;
mod parser;
mod registers;
mod simulator;
mod tokens;

pub use crate::simulator::Simulator;
