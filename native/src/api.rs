pub fn times_from_rust(left: usize, right: usize) -> usize {
    left * right
}

pub fn hello_from_rust(count: usize) -> String {
    let mut s = "".to_string();
    for _ in 0..count {
        s += "Hello from Rust."
    }
    println!("{}, {}", s, count);
    s
}
