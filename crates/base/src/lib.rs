pub mod app;
pub use app::{App, APP};

pub fn init_app() {
    let mut app = APP.write().unwrap();
    app.fetch_status();
    app.log("main");
    println!("LEN: {}", app.get_diff().len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_run() {
        init_app();
    }
}
