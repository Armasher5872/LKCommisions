#![feature(proc_macro_hygiene)]

mod custom;

#[skyline::main(name = "lkcommisions")]
pub fn main() {
    custom::install();
}