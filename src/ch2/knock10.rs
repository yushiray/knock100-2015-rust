use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;

use std::io;

pub fn count_lines(path: &Path) -> usize {
    let file = File::open(path).expect("file not found");
    let buf = BufReader::new(file);
    buf.lines().count()
}

pub fn tab_to_space(path: &Path) {
    let file = File::open(path).expect("file not found");
    let buf = BufReader::new(file);
    for line in buf.lines() {
        let line = line.unwrap();
        println!("{}", &line.replace("\t", " "));
    }
}

pub fn tab_to_space2(path: &Path, tab_width: usize) -> io::Result<String> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let spaces = " ".repeat(tab_width);
    Ok(br.lines().map(|s| match s { Ok(s) => s.replace("\t", &spaces) + "\n", Err(_) => "\0".to_string() }).collect())
}

pub fn cut(input_file_name: &Path, num: usize, output_file_name: &str) {
    let input_f = File::open(input_file_name).expect("file not found");
    let read_buf = BufReader::new(input_f);
    let mut output_f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file_name)
        .expect(format!("can't open file[{}] with write option", output_file_name).as_str());
    read_buf.lines().for_each(|line| match line {
        Ok(line) => {
            let columns: Vec<_> = line.split('\t').collect();
            writeln!(output_f, "{}", columns[num]).unwrap();
            output_f.flush().expect("Error during flush");
        }
        Err(_) => panic!("parse error "),
    });
}