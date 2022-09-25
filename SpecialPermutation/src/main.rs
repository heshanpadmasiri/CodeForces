use std::io;

const DEBUG:bool = false;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Expect t");
    let t: i64 = t.trim().parse().expect("Expect t to be an integer");

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    let nums = line.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();
    let n = nums[0];
    let a = nums[1];
    let b = nums[2];
    let mut lhs = Vec::<i64>::new();
    let mut rhs = Vec::<i64>::new();
    let mut any = Vec::<i64>::new();
    
    lhs.push(a); // min(lhs) == a;
    rhs.push(b); // max(rhs) == b;
    
    if DEBUG {
        println!("--lhs {:?} rhs {:?} any {:?}", lhs, rhs, any);
    }
    for i in 1 ..=n {
        if DEBUG {
            print!("--i {i} ");
        }
        if i == a || i == b {
            if DEBUG {
                println!("skipped");
                println!("--lhs {:?} rhs {:?} any {:?}", lhs, rhs, any);
            }
            continue;
        }
        if i > a && i > b {
            lhs.push(i);
            if DEBUG {
                println!("add to lhs");
                println!("--lhs {:?} rhs {:?} any {:?}", lhs, rhs, any);
            }
        }
        else if i < b && i < a {
            rhs.push(i);
            if DEBUG {
                println!("add to rhs");
                println!("--lhs {:?} rhs {:?} any {:?}", lhs, rhs, any);
            }
        }
        else if i > a && i < b {
            any.push(i);
            if DEBUG {
                println!("add to any");
                println!("--lhs {:?} rhs {:?} any {:?}", lhs, rhs, any);
            }
        }
        else {
            if DEBUG {
                println!("a {i}");
            }
            println!("-1");
            return;
        }
    }
    if DEBUG {
        println!("lhs {:?} rhs {:?} any {:?}", lhs, rhs, any);
    }
    let target_len = (n/2) as usize;
    if lhs.len() > target_len || rhs.len() > target_len {
            println!("-1");
            return;
    }
    for each in any {
        if lhs.len() < target_len {
            lhs.push(each);
        }
        else if rhs.len() < target_len {
            rhs.push(each);
        }
        else {
            if DEBUG {
                println!("b");
            }
            // this shouldn't happen?
            println!("-1");
            return;
        }
    }
    let lhs = lhs.into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
    let rhs = rhs.into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
    println!("{lhs} {rhs}");
}

