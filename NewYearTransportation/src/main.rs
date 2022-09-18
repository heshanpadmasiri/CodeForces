use std::io;
use std::collections::HashMap;

fn main() {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Failed to read dataline");
    let tmp = data.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();
    let n = tmp[0];
    let t = tmp[1];

    data = String::new();
    io::stdin().read_line(&mut data).expect("Failed to read portal data");
    let portals = data.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();

    let mut map = HashMap::new();
    for i in 1..n+1 {
        map.insert(i, Vec::<i64>::new());
    }
    for (index, offset) in portals.iter().enumerate() {
        let start = (index as i64) + 1; 
        let destination = *offset + start;
        map.get_mut(&start).expect("unexpected").push(destination);
    }

    // debug map
    // for (start, destinations) in &map {
    //     println!("{start} -> {:?}", destinations);
    // }
    if dfs(&map, 1, t) {
        println!("YES");
    }
    else {
        println!("NO");
    }
}

fn dfs(map: &HashMap::<i64, Vec::<i64>>, current: i64, t: i64) -> bool {
    if current == t {
        return true;
    } 
    else {
        let child_nodes = map.get(&current).expect("unexpected 1");
        // debug
        // println!("current: {current}, child_nodes: {child_nodes:?}");
        for child_node in child_nodes {
            if dfs(map, *child_node, t) {
                return true;
            }
        }     
    }
    return false;
}
