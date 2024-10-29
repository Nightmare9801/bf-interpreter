pub mod runner;
use std::{env::{self}, fs::File, io::{BufReader, Read}};

use crate::runner::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path: String = args[1].clone();

    let mut data: String = String::new();
    let file: File = File::open(path).unwrap();
    let mut reader: BufReader<File> = BufReader::new(file);
    reader.read_to_string(&mut data).unwrap();

    let mut interpreter = Interpreter::new();

    interpreter.run(&data);

}
