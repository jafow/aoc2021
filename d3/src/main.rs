use std::cmp::Ordering;
use common::{input, input_to_slice_lines};

// const WORDSIZE: usize = 12;
const WORDSIZE: usize = 5;

#[derive(Debug, PartialEq)]
struct DiagCode {
    x: u32,
    y: u32,
}

// fn frequencies(xs: &Vec<&str>) -> Vec<(u32, u32, Option(&str))> {
fn frequencies(xs: &Vec<&str>) -> Vec<DiagCode> {
    let mut freqs: Vec<DiagCode> = Vec::with_capacity(WORDSIZE);
    // prepare the freq list
    for _ in 0..WORDSIZE {
        // freqs.push(( 0u32, 0u32, None ));
        freqs.push(DiagCode { x: 0u32, y: 0u32 });
    }
    
    for x in xs {
        // collect the bits in each line
        let bits = x.chars().map(|c| c.to_digit(2).unwrap()).collect::<Vec<u32>>();

        // construct frequency
        for (idx, bit) in bits.iter().enumerate() {
            match bit {
                0 => {
                    let item = &freqs[idx];
                    freqs[idx] = DiagCode { x: item.x + 1, y: item.y };
                },
                1 => {
                    let item = &freqs[idx];
                    // freqs[idx] = (item.x, item.y + 1);
                    freqs[idx] = DiagCode { x: item.x , y: item.y + 1 };
                },
                _ => {} 
            }
        }
    }
    freqs
}

#[test]
fn test_freqs () {
    let xs = vec!["00100","11110", "10110"];
    assert_eq!(frequencies(&xs), vec![
               DiagCode { x: 1, y: 2 },
               DiagCode { x: 2, y: 1 },
               DiagCode { x: 0, y: 3 },
               DiagCode { x: 1, y: 2 },
               DiagCode { x: 3, y: 0 },
    ]);

    let xs = vec![
        "00100", 
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010"];
    assert_eq!(frequencies(&xs), vec![
               DiagCode { x: 5, y: 7 },
               DiagCode { x: 7, y: 5 },
               DiagCode { x: 4, y: 8 },
               DiagCode { x: 5, y: 7 },
               DiagCode { x: 7, y: 5 },
    ]);

}

fn mul_strs(p: &str, q: &str) -> Option<u32> {
    let pint = u32::from_str_radix(p, 2u32).expect("ok");
    let qint = u32::from_str_radix(q, 2u32).expect("ok");
    Some(pint * qint)
}

fn f_remain(idx: usize, val: u32, xs: &Vec<String>) -> Vec<String> {
    dbg!("=== fremain === input res {:?} on idx {:?} with valur: {:?}", &xs, &idx, &val);
    if xs.len() == 1 {
        dbg!("we got to the end at idx {:?}, res {:?}", &idx, xs);
        return xs.to_vec();
    }
    let mut res: Vec<String> = Vec::new();
    for x in xs {
        let xidx = x[idx..idx+1].parse::<u32>().unwrap();
        if xidx == val {
            dbg!("this value matches at idx {:?}", xidx);
            res.push(x.to_string());
        }
    }
    dbg!("=== fremain === returning res {:?} on idx {:?}", &res, &idx);
    res
}


fn p1(xs: &Vec<&str>) -> Option<u32> {
    let freqtable = frequencies(xs);
    let gamma = freqtable.iter().map(|item| {
        if item.x > item.y {
            '0'
        } else {
            '1'
        }}).collect::<String>();

    let epsil = freqtable.iter().map(|item| {
        if item.x > item.y {
            '1'
        } else {
            '0'
        }}).collect::<String>();

    let res: Option<u32> = mul_strs(&gamma, &epsil);
    res
}




fn p2(xs: &Vec<&str>) -> Option<String> {
    let freqtable = frequencies(xs);
    dbg!(&freqtable.len());

    // starting vec
    let mut rem: Vec<String> = Vec::new();
    for x in xs {
        rem.push(x.to_string())
    }

    for (idx, diagcode) in freqtable.iter().enumerate() {
        let max: u32;
        match diagcode.x.cmp(&diagcode.y) {
            Ordering::Equal => {
               dbg!("p2::: equal====== idx: {:?}, diagcode {:?}", &idx, &diagcode);
               max = 1u32;
            },
            Ordering::Less => {
               dbg!("p2::: less ====== idx: {:?}, diagcode {:?}", &idx, &diagcode);
               max = 1u32;
            },
            Ordering::Greater => {
               dbg!("p2::: greater ====== idx: {:?}, diagcode {:?}", &idx, &diagcode);
               max = 0u32;
            }
        }
        rem = f_remain(idx, max, &rem);
    }
    let p = &rem[0];
    dbg!(&rem);
    Some(p.to_string())
}

#[test]
fn test_p2() {
    let xs = vec!["00100","11110", "10110","10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"];
    assert_eq!(p2(&xs), Some(String::from("10111")));
}



fn main() {
    let raw: String = input();
    let xs = input_to_slice_lines(&raw);

    let res = p1(&xs);
    println!("{:?}", res);

    let res = p2(&xs);
    println!("{:?}", res);
}
