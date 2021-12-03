use std::io::{self, Read};

pub fn input() -> String {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).expect("read input");
    s
}

pub fn input_to_vec_ints(input: &str) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    for line in input.lines() {
        let x: i32 = line.parse().expect("caste to i32");
        res.push(x);
    }
    res
}

pub fn stdin_to_vec_ints() -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let stdin: String = input();

    for line in stdin.lines() {
        let x: i32 = line.parse().expect("caste to i32");
        res.push(x);
    }
    res
}

pub fn input_to_slice_lines(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
}

