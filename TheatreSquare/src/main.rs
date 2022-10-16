use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    let line = line.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();

    let n = line[0];
    let m = line[1];
    let a = line[2];

    let vertical_count = if n % a == 0 { n/a } else { (n/a)+1 };
    let horizontal_count = if m % a == 0 { m/a } else { (m/a)+1 };
    println!("{}", vertical_count*horizontal_count);
}
