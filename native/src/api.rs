use flutter_rust_bridge::frb;

use base;
pub use base::git_utils::{DiffLine, DiffLineType};

pub fn get_repo() -> String {
    base::get_repo()
}

pub fn set_repo(path: String) {
    base::set_repo(path);
}

pub fn open_default_repo() {
    base::open_default_repo();
}

pub fn get_default_repo() -> String {
    base::get_default_repo()
}

pub fn update() {
    base::update();
}

#[frb(mirror(DiffLineType))]
pub enum _DiffLineType {
    None,
    Header,
    Add,
    Delete,
}

#[frb(mirror(DiffLine))]
pub struct _DiffLine {
    pub content: String,
    pub line_type: DiffLineType,
}

pub fn get_diff() -> Vec<DiffLine> {
    base::get_diff()
}

pub fn get_status_items() -> Vec<String> {
    base::get_status_items()
}

pub fn set_selection(index: i32, stage: bool) {
    base::set_selection(index, stage);
}

pub fn get_index_items() -> Vec<String> {
    base::get_index_items()
}

pub fn index_update() {
    base::index_update();
}

pub fn commit(msg: String) -> bool {
    base::commit(msg)
}
