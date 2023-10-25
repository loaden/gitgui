mod app;
pub use app::APP;
use git_utils::DiffLine;
mod git_status;

pub mod git_utils;

pub fn get_repo() -> String {
    let app = APP.read().unwrap();
    app.repo_path()
}

pub fn set_repo(path: String) {
    let mut app = APP.write().unwrap();
    app.set_repo(path);
}

pub fn open_default_repo() {
    let mut app = APP.write().unwrap();
    let repo = app.get_default_repo();
    app.set_repo(repo);
}

pub fn get_default_repo() -> String {
    let app = APP.read().unwrap();
    app.get_default_repo()
}

pub fn fetch_status() {
    let mut app = APP.write().unwrap();
    app.fetch_status();
}

pub fn get_diff() -> Vec<DiffLine> {
    let app = APP.read().unwrap();
    app.get_diff()
}

pub fn get_status_items() -> Vec<String> {
    let app = APP.read().unwrap();
    app.get_status_items()
}

pub fn set_status_select(index: usize) {
    let mut app = APP.write().unwrap();
    app.set_status_select(index);
}

pub fn get_index_items() -> Vec<String> {
    let app = APP.read().unwrap();
    app.get_index_items()
}

pub fn index_add() {
    let mut app = APP.write().unwrap();
    app.index_add();
}

pub fn commit(msg: String) {
    let app = APP.read().unwrap();
    app.commit(msg);
}

#[cfg(test)]
mod tests {
    #[test]
    fn src_path() {
        use std::path::PathBuf;
        use crate::app::App;
        let mut path = PathBuf::from("foo.git");
        path.pop();
        assert!(path.parent().is_none());
        let mut src = App::src_path();
        src.push(".git");
        assert!(src.exists());
    }

    #[test]
    fn init_app() {
        super::open_default_repo();
    }

    #[test]
    fn fetch_status() {
        super::fetch_status();
    }
}
