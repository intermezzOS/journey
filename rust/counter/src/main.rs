use std::env;
use std::process;

fn main() {
    let count = env::args().count();
    process::exit((count - 1) as i32)
}
