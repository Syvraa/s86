#![deny(clippy::pedantic)]

mod instruction;
mod label_parser;
mod lexer;
mod operands;
mod parser;
mod registers;
mod simulator;
mod tokens;

pub use crate::simulator::Simulator;
