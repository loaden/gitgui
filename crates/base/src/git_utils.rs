use git2::{
    DiffFormat, DiffOptions, IndexAddOption, Repository, Status, StatusOptions,
    StatusShow,
};
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

pub fn repo(path: &str) -> Repository {
    let repo = Repository::init(path).unwrap();

    if repo.is_bare() {
        panic!("bare repo")
    }

    repo
}

pub fn get_diff(r: String, p: &Path, stage: bool) -> Diff {
    let repo = repo(&r);

    let mut opt = DiffOptions::new();
    opt.pathspec(p);

    let diff = if !stage {
        // diff against stage
        repo.diff_index_to_workdir(None, Some(&mut opt)).unwrap()
    } else {
        // diff against head
        let ref_head = repo.head().unwrap();
        let parent = repo.find_commit(ref_head.target().unwrap()).unwrap();
        let tree = parent.tree().unwrap();
        repo.diff_tree_to_index(
            Some(&tree),
            Some(&repo.index().unwrap()),
            Some(&mut opt),
        )
        .unwrap()
    };

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

pub fn commit(r: String, msg: String) {
    let repo = repo(&r);
    let signature = repo.signature().unwrap();

    let reference = repo.head().unwrap();
    let mut index = repo.index().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    let parent = repo.find_commit(reference.target().unwrap()).unwrap();

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        msg.as_str(),
        &tree,
        &[&parent],
    )
    .unwrap();
}

pub fn index_add(r: String, file: String) -> bool {
    let repo = repo(&r);
    let mut index = repo.index().unwrap();

    let path = Path::new(file.as_str());

    let cb = &mut |p: &Path, _matched_spec: &[u8]| -> i32 {
        if p == path {
            0
        } else {
            1
        }
    };

    if let Ok(_) = index.add_all(
        path,
        IndexAddOption::DISABLE_PATHSPEC_MATCH | IndexAddOption::CHECK_PATHSPEC,
        Some(cb),
    ) {
        index.write().unwrap();
        return true;
    }
    false
}

pub fn index_empty(r: String) -> bool {
    let repo = repo(&r);

    let statuses = repo
        .statuses(Some(StatusOptions::default().show(StatusShow::Index)))
        .unwrap();

    statuses.is_empty()
}
