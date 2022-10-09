use std::io;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("expect t");
    let t: i64 = t.trim().parse().expect("expect t to be integer");
    
    for _ in 0..t {
        println!("{}", solve()); 
    }
}

fn solve() -> i64 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    let nums = line.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();

    let n = nums[0];
    let k = nums[1];
    if k == 1 {
        return 1;
    }
    // println!("n: {n} k: {k}");
    return binary_search(1, k, is_k_divisible(n, k));
    // let result = binary_search(1, k, is_k_divisible(n,k));
    // println!("");
    // return result;
}

fn binary_search<F>(start: i64, end: i64, predicate: F) -> i64 where F: Fn(i64)->bool {
    // print!("\t{start} - {end}");
    if start != end {
        let middle = (start + end) / 2;
        if middle == start || middle == end {
            // we have only two values (start, end);
            if predicate(start) {
                return start;
            }
            else if predicate(end) {
                return end; 
            }
            panic!("no satisfying value found");
        }
        // println!("-- {middle}");
        if predicate(middle) {
            return binary_search(start, middle, predicate);
        }
        else {
            return binary_search(middle, end, predicate);
        }
    }
    panic!("unexpected"); // eror here
}

fn is_k_divisible(n: i64, k: i64) -> impl Fn(i64) -> bool {
    return move |m:i64| {
        let min_sum = (n-1) + m;
        let max_sum = n * m;
        if min_sum % k == 0 || max_sum % k == 0 {
            return true;
        }
        let min_factor = min_sum / k;
        let max_factor = max_sum / k;
        return min_factor != max_factor;
    }
} 
