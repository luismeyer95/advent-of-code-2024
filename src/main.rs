#![feature(binary_heap_drain_sorted)]
#![feature(let_chains)]

mod d1;
mod d10;
mod d11;
mod d12;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;
mod utils;

fn main() {
    let result = d12::main::solution2(utils::file_line_iter("./text/d12")).unwrap();
    println!("{}", result);
}
