use base;
pub use base::app::{DiffLine, DiffLineType};
use flutter_rust_bridge::frb;

pub fn app_run() {
    base::init_app();
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

pub fn fetch_status() {
    base::fetch_status();
}

pub fn update_diff() {
    base::update_diff();
}

pub fn get_diff() -> Vec<DiffLine> {
    base::get_diff()
}
