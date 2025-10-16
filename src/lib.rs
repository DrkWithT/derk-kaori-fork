#![feature(explicit_tail_calls)]
#![allow(incomplete_features)]

pub mod bytecode;
pub mod cfg_ir;
pub mod error;
pub mod lexer;
pub mod program;
pub mod semantic;
pub mod syntax;
pub mod virtual_machine;
