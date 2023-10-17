use base;

pub fn times_from_rust(left: usize, right: usize) -> usize {
    left * right
}

pub fn hello_from_rust(count: usize) -> String {
    let mut s = "".to_string();
    for _ in 0..count {
        s += "hi Rust."
    }
    println!("{}, {}", s, count);
    s
}

pub fn get_diff() -> String {
    let app = base::APP.read().unwrap();
    app.get_diff()
}

pub fn app_run() {
    base::init_app();
}