#![feature(binary_heap_drain_sorted)]
#![feature(let_chains)]

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
// mod d08;
// mod d09;
mod utils;

fn main() {
    let result = d07::main::solution2(utils::file_line_iter("./text/d07")).unwrap();
    println!("{}", result);
}
