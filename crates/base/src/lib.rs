pub mod app;
pub use app::{App, DiffLine, APP};

pub fn init_app() {
    let app = APP.read().unwrap();
    app.log("INIT DONE!".to_string());
}

pub fn get_repo() -> String {
    match App::repo().to_str() {
        Some(s) => s.to_string(),
        None => String::new(),
    }
}

pub fn fetch_status() {
    let mut app = APP.write().unwrap();
    app.fetch_status();
    app.log(format!("fetch_status: {}", app.get_diff().len()));
}

pub fn update_diff() {
    let mut app = APP.write().unwrap();
    app.update_diff();
    app.log(format!("update_diff: {}", app.get_diff().len()));
}

pub fn get_diff() -> Vec<DiffLine> {
    let app = APP.read().unwrap();
    app.log(format!("get_diff: {}", app.get_diff().len()));
    app.get_diff()
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_app() {
        super::init_app();
    }

    #[test]
    fn fetch_status() {
        super::fetch_status();
    }

    #[test]
    fn update_diff() {
        super::fetch_status();
        super::update_diff();
    }
}
