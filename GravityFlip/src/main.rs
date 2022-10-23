use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read n");
    let n: usize = n.trim().parse().expect("Expect n to be integer");
    
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    let mut config = line.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();
    for i in (0..n-1).rev() { 
        for j in i+1..n {
            let current = j;
            let prev = j-1;
            if config[current] < config[prev] {
                let diff = config[prev] - config[current];
                config[current] += diff;
                config[prev] -= diff;
            }
            else {
                break;
            }
        }
    }
    let output = config.iter().map(i64::to_string).collect::<Vec<_>>().join(" ");
    println!("{output}");
}
