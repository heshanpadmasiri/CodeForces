use std::{io, cmp};

fn main() {
    let t = read_int();
    for _ in 0..t {
        println!("{}", solve());        
    }
}

fn solve() -> usize {
    let data = read_array();
    let a = data[0];
    let b = data[1];
    let x = data[2];
    let y = data[3];
    let n = data[4];
    let ans1 = if a-x > n {
        (a-n) * b
    } else {
        let leftover = n - (a-x);
        let rhs = if leftover < b && b - leftover > y {
            b - leftover
        } else {
            y
        };
        x * rhs
    };
    let ans2 = if b-y > n {
        (b-n) * a
    } else {
        let leftover = n - (b-y);
        let lhs = if leftover < a && a - leftover > x {
            a - leftover
        } else {
            x
        };
        lhs * y
    };
    cmp::min(ans1, ans2)
}

fn read_int() -> usize {
    let line = read_line();
    return line.trim().parse().expect("expect an integer");
}

fn read_array() -> Vec<usize> {
    let line = read_line();
    return line.trim().split(' ').flat_map(str::parse::<usize>).collect::<Vec<_>>();
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    return line;
}
