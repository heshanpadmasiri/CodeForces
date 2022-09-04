use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read n");
    let _n: i64 = n.trim().parse().expect("Expect n to be integer");
    
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    let nums = line.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();

    let sum: i64 = nums.iter().sum();

    println!("{}", sum);
}
