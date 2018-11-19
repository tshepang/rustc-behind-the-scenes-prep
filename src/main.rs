#![no_std]

extern crate hifive;

pub fn square(num: u32) -> u32 {
    num * num
}

fn main() {
    square(1983);
}
