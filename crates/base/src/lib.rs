pub mod app;
pub use app::{App, APP};

pub fn init_app() {
    let mut app = APP.lock().unwrap();
    app.fetch_status();
    app.log("main");
}

#[cfg(test)]
mod tests {}
