use std::cmp::Ordering;
use common::{input, input_to_slice_lines};

const WORDSIZE: usize = 12;

#[derive(Debug, PartialEq)]
struct DiagCode {
    x: u32,
    y: u32,
}

enum BitCriteria {
    O2,
    CO2
}

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

fn p2_inner<'a>(xs: &[&'a str], idx: usize, bc: BitCriteria) -> Vec<&'a str> {
    if xs.len() == 1 {
        return xs.to_vec();
    }

    // the indexes of each 0 (z) and 1 (o)
    let mut z: Vec<usize> = Vec::new();
    let mut o: Vec<usize> = Vec::new();
    
    for i in 0..xs.len() {
        match &xs[i][idx..idx+1] {
            "0" => {
                z.push(i);
            },
            "1" => {
                o.push(i);
            },
            _ => ()
        }
    }
    // which are greater?
    match z.len().cmp(&o.len()) {
        Ordering::Equal => {
            match bc {
                BitCriteria::O2 => xs.iter().enumerate().filter(|(i, _)| { o.contains(i) }).map(|t| *t.1).collect::<Vec<_>>(),
                BitCriteria::CO2 => xs.iter().enumerate().filter(|(i, _)| { z.contains(i) }).map(|t| *t.1).collect::<Vec<_>>()
            }
        }
        Ordering::Less => {
            match bc {
                BitCriteria::O2 => xs.iter().enumerate().filter(|(i, _)| { o.contains(i) }).map(|t| *t.1).collect::<Vec<_>>(),
                BitCriteria::CO2 => xs.iter().enumerate().filter(|(i, _)| { z.contains(i) }).map(|t| *t.1).collect::<Vec<_>>()
            }
        }
        Ordering::Greater => {
            match bc {
                BitCriteria::O2 => xs.iter().enumerate().filter(|(i, _)| { z.contains(i) }).map(|t| *t.1).collect::<Vec<_>>(),
                BitCriteria::CO2 => xs.iter().enumerate().filter(|(i, _)| { o.contains(i) }).map(|t| *t.1).collect::<Vec<_>>()
            }
        }
    }
}

fn p2<'a>(xs: &[&'a str]) -> Option<u32> {
    let mut o2 = p2_inner(xs, 0usize, BitCriteria::O2);
    let mut co2 = p2_inner(xs, 0usize, BitCriteria::CO2);
    for w in 1..WORDSIZE {
        o2 = p2_inner(&o2, w, BitCriteria::O2);
        co2 = p2_inner(&co2, w, BitCriteria::CO2);
    }
    let res = mul_strs(o2[0], co2[0]);
    res
}

#[test]
fn test_p2() {
    const WORDSIZE: usize = 5;
    let xs = vec!["00100","11110", "10110","10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"];
    assert_eq!(p2(&xs), Some(230u32));
}



fn main() {
    let raw: String = input();
    let xs = input_to_slice_lines(&raw);

    let res = p1(&xs);
    println!("p1 {:?}", res);

    let res = p2(&xs);
    println!("p2: {:?}", res);
}
