use git2::{DiffFormat, DiffOptions, Repository, Status};
use std::path::Path;

#[derive(Copy, Clone, PartialEq)]
pub enum DiffLineType {
    None,
    Header,
    Add,
    Delete,
}

impl Default for DiffLineType {
    fn default() -> Self {
        DiffLineType::None
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct DiffLine {
    pub content: String,
    pub line_type: DiffLineType,
}

#[derive(Default, PartialEq)]
pub struct Diff(pub Vec<DiffLine>);

pub fn on_index(s: &Status) -> bool {
    s.is_index_new() || s.is_index_modified()
}

pub fn get_repo(path: String) -> Repository {
    let repo = Repository::init(path).unwrap();

    if repo.is_bare() {
        panic!("bare repo")
    }

    repo
}

pub fn get_diff(repo: String, path: &Path) -> Diff {
    let repo = get_repo(repo);

    let mut opt = DiffOptions::new();
    opt.pathspec(path);
    let diff = repo.diff_index_to_workdir(None, Some(&mut opt)).unwrap();

    let mut res = Vec::new();
    diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
        let line_type = match line.origin() {
            'H' => DiffLineType::Header,
            '<' | '-' => DiffLineType::Delete,
            '>' | '+' => DiffLineType::Add,
            _ => DiffLineType::None,
        };

        let diff_line = DiffLine {
            content: String::from_utf8_lossy(line.content()).to_string(),
            line_type,
        };

        res.push(diff_line);
        true
    })
    .unwrap();

    Diff(res)
}
