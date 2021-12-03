use common::{input, input_to_slice_lines};

#[derive(Debug)]
struct Position {
    hpos: i32,
    vpos: i32
}

enum Direction {
    Up,
    Down,
    Forward,
    Noop
}


fn position(x: Vec<&str>) -> (Direction, i32) {
    let direction = x[0];
    let p = x[1].parse::<i32>().unwrap();

    match direction {
        "up" => (Direction::Up, p),
        "down" => (Direction::Down, p),
        "forward" => (Direction::Forward, p),
        _ => (Direction::Noop, 0)
    }
}

fn p1(xs: &Vec<&str>) -> Position {
    let tups = xs.iter().map(|line| line.split(" ").collect::<Vec<&str>>());
    let mut hpos = 0i32;
    let mut vpos = 0i32;

    for direction in tups {
        let (dir, amount) = position(direction);
        match dir {
            Direction::Up => {
                vpos = vpos - amount
            },
            Direction::Down => {
                vpos = vpos + amount
            },
            Direction::Forward => {
                hpos = hpos + amount
            },
            Direction::Noop => {}
        }
    }
    Position { hpos, vpos }
}

fn p2(xs: &Vec<&str>) -> Position {
    let tups = xs.iter().map(|line| line.split(" ").collect::<Vec<&str>>());
    let mut hpos = 0i32;
    let mut vpos = 0i32;
    let mut aim = 0i32;

    for direction in tups {
        let (dir, amount) = position(direction);
        match dir {
            Direction::Up => {
                aim = aim - amount
            },
            Direction::Down => {
                aim = aim + amount
            },
            Direction::Forward => {
                hpos = hpos + amount;
                vpos = vpos + (aim * amount);
            },
            Direction::Noop => {}
        }
    }
    Position { hpos, vpos }
}

fn main() {
    let raw: String = input();
    let xs = input_to_slice_lines(&raw);

    let res = p1(&xs);
    println!("{:?}", res);
    println!("p1: {}", res.hpos * res.vpos);
    let res = p2(&xs);
    println!("{:?}", res);
    println!("p2: {}", res.hpos * res.vpos);

}
