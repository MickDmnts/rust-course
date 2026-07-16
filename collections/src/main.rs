use std::collections::{HashMap, hash_map};

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);
    let x: Option<i32> = v.pop();
    println!("index 1 is {}", v[1]);

    let mut v_fast: Vec<i32> = vec![2, 4, 6];

    let mut h: HashMap<u8, bool> = HashMap::new();
}
