#![deny(clippy::pedantic)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]

pub mod diff;
mod instruction;
mod label_parser;
mod lexer;
mod operands;
mod parser;
mod registers;
mod simulator;
mod simulator_error;
mod syntax_error;
mod tokens;

pub use crate::simulator::Simulator;
