use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    let tmp = line.trim().split(' ').flat_map(str::parse::<usize>).collect::<Vec<_>>();
    let k = tmp[1];

    line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    let mut speeds = line.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();
    
    speeds.sort_unstable_by(|a, b| b.cmp(a));
    let ans = speeds[k-1]; 
    println!("{ans}");
}
