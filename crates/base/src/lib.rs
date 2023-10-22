pub mod app;
pub use app::{App, DiffLine, APP};

pub fn init_app() {
    let app = APP.read().unwrap();
    app.log("INIT DONE!");
}

pub fn get_repo() -> String {
    match App::src_path().to_str() {
        Some(s) => s.to_string(),
        None => String::new(),
    }
}

pub fn fetch_status() {
    let mut app = APP.write().unwrap();
    app.fetch_status();
}

pub fn update_diff() {
    let mut app = APP.write().unwrap();
    app.update_diff();
}

pub fn get_diff() -> Vec<DiffLine> {
    let app = APP.read().unwrap();
    app.get_diff()
}

#[cfg(test)]
mod tests {
    #[test]
    fn src_path() {
        use std::path::PathBuf;
        let mut path = PathBuf::from("foo.git");
        path.pop();
        assert!(path.parent().is_none());
        let mut src = super::App::src_path();
        src.push(".git");
        assert!(src.exists());
    }

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
