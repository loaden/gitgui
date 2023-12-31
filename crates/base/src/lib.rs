mod app;
pub use app::APP;

mod git_index;
mod git_status;

pub mod git_utils;
use git_utils::DiffLine;

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

pub fn update() {
    let mut app = APP.write().unwrap();
    app.update();
}

pub fn get_diff() -> Vec<DiffLine> {
    let app = APP.read().unwrap();
    app.get_diff()
}

pub fn get_status_items() -> Vec<String> {
    let app = APP.read().unwrap();
    app.get_status_items()
}

pub fn set_selection(index: i32, stage: bool) {
    let mut app = APP.write().unwrap();
    app.set_selection(index, stage);
}

pub fn get_index_items() -> Vec<String> {
    let app = APP.read().unwrap();
    app.get_index_items()
}

pub fn index_update() {
    let mut app = APP.write().unwrap();
    app.index_add_remove();
}

pub fn commit(msg: String) -> bool {
    let app = APP.read().unwrap();
    app.commit(msg)
}

#[cfg(test)]
mod tests {
    #[test]
    fn src_path() {
        use crate::app::App;
        use std::path::PathBuf;
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
    fn update() {
        super::update();
    }

    #[test]
    fn get_diff() {
        super::open_default_repo();
        super::get_diff();
    }

    #[test]
    fn set_status_select() {
        super::open_default_repo();
        super::set_selection(1, false);
        super::set_selection(0, false);
    }
}
