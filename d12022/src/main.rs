use common::{input, input_to_slice_lines};

fn p1(xs: &Vec<&str>) -> Option<u32> {
    let mut max: u32 = 0;
    let mut p: u32 = 0;

    for x in xs {
        if p > max {
            max = p
        }
        if x.is_empty() {
            p = 0u32;
        } else {
            p += x.parse::<u32>().unwrap();
        }
    }
    Some(max)
}

fn p2(xs: &Vec<&str>) -> Option<u32> {
    let mut elves: Vec<Vec<u32>> = vec![];
    let mut chunks: Vec<u32> = vec![];

    for x in xs {
        if x.is_empty() {
            elves.push(chunks);
            chunks = vec![];
        } else {
            chunks.push(x.parse::<u32>().unwrap())
        }
    }

    let mut flat: Vec<u32> = vec![];
    for chunk in elves {
        flat.push(chunk.iter().sum())
    }
    let _ = &flat.sort();

    let first = flat.pop()?;
    let sec = flat.pop()?;
    let thir = flat.pop()?;

    Some(first + sec + thir)
}

fn main() {
    let raw: String = input();
    let xs = input_to_slice_lines(&raw);
    let res = p1(&xs);
    dbg!(res.unwrap());
    let res = p2(&xs);
    dbg!(res.unwrap());
}
