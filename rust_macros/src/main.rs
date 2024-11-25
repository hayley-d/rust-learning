// Declarative Macros are like match statements
//
//Procedural macros are like functions
use rust_macros::macros::*;
use rust_macros::pmac::*;

use rust_macros::hello_macro::HelloMacro;
use rust_macros_derive::hello_macro::::HelloMacro;

fn main() {
    Pancakes::hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;
