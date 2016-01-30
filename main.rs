use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = match env::args().nth(1) {
        Some(n) => n,
        None => panic!("Must give file name as command line argument")
    };

    let contents = read(&filename);
    let reversed = contents.chars().rev().collect::<String>();

    write(&filename, &reversed);
}

fn read(filename: &String) -> String {
    let mut file = File::open(filename).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    string
}

fn write(filename: &String, string: &String) {
    let mut f = File::create(filename).unwrap();
    f.write_all(string.clone().into_bytes().as_ref()).unwrap();
}
