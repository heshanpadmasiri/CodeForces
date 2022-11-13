use std::io;

fn main() {
    let t = read_int();
    for _ in 0..t {
        println!("{}", solve())        
    }
}

fn solve() -> i64 {
    let n = read_int() as usize;
    let mut values = read_array();
    if n == 1 {
        return values[0];
    }
    values.sort_unstable();
    let mut d = values[0];
    let mut ans = values[0];
    for i in 1..n {
        let current = values[i] - d;
        if current > ans {
            ans = current;
        }
        d += current;
    }
    ans
}

fn read_int() -> i64 {
    let line = read_line();
    return line.trim().parse().expect("expect an integer");
}

fn read_array() -> Vec<i64> {
    let line = read_line();
    return line.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    return line;
}
