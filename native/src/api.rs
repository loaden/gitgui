use base;
pub use base::app::{DiffLine, DiffLineType};
use flutter_rust_bridge::frb;

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

pub fn fetch_status() {
    base::fetch_status();
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
