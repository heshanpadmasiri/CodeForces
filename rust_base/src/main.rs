use std::io;

fn main() {
    let n = read_int();
    let nums = read_array();
    let sum: i64 = nums.iter().sum();

    println!("{}", sum);
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
