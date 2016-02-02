// Based on: https://doc.rust-lang.org/book/no-stdlib.html
#![feature(lang_items)]
#![feature(start)]
#![no_std] // no standard lib linking
#![no_main] // there is no rust main function only a C-like extern function

extern crate libc;

#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern fn main(argc: i32, _argv: *const *const u8) -> i32 { // main receives command line arguments using the C ABI
    // Number of arguments passed to binary is equal to argc - 1
    argc - 1
}
