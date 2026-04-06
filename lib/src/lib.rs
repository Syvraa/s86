#![deny(clippy::pedantic)]

mod diff;
mod instruction;
mod label_parser;
mod lexer;
mod operands;
mod parser;
mod registers;
mod simulator;
mod simulator_error;
mod tokens;

pub use crate::simulator::Simulator;
