use std::io;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Expect t");
    let t: i64 = t.trim().parse().expect("Expect t to be an integer");
    
    for _ in 0 .. t {
        solve();
    }
}

fn solve() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Expect n");
    let n: usize = n.trim().parse().expect("Expect n to be an integer");

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    let p = line.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();

    let mut data = p.iter().enumerate().map(|(i, x)| (x, i as i64)).collect::<Vec<_>>();
    data.sort();
    // println!("-- {:?}", data);
    let starting_indices = vec![data[0].1, data[1].1];
    let mut i;
    let mut k;
    if starting_indices[0] > starting_indices[1] {
        i = starting_indices[1];
        k = starting_indices[0];
    }
    else {
        i = starting_indices[0];
        k = starting_indices[1];

    }
    // println!("-- {i}, {k}");
    for j in 2 .. n {
        let index = data[j].1;
        if index > i &&  index < k {
            println!("YES");
            println!("{} {} {}", i+1, index + 1, k+1);
            return;
        }
        else if index < i {
            i = index;
        }
        else if index > k {
            k = index;
        }
        // println!(" -- {i}, {k}");
    }
    println!("NO");
}
