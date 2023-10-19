use base;
pub use base::app::{DiffLine, DiffLineType};
use flutter_rust_bridge::{frb, ZeroCopyBuffer};

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

pub fn get_diff() -> Vec<DiffLine> {
    let app = base::APP.read().unwrap();
    app.get_diff()
}
