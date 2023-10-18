use git2::{DiffFormat, Repository, Status};
use std::path::Path;
use std::sync::RwLock;

lazy_static::lazy_static! {
    pub static ref APP: RwLock<App> = RwLock::new(App::default());
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Default, PartialEq)]
pub struct DiffLine {
    content: String,
    line_type: DiffLineType,
}

#[derive(Debug, Default, PartialEq)]
pub struct Diff(Vec<DiffLine>);

#[derive(Default)]
pub struct App {
    status_items: Vec<String>,
    status_select: Option<usize>,
    index_items: Vec<String>,
    diff: Diff,
    offset: u16,
    do_quit: bool,
    count: u32,
}

impl App {
    pub fn is_quit(&self) -> bool {
        self.do_quit
    }

    pub fn log(&self, msg: &str) {
        println!("COUNT: {}, {}", self.count, msg);
    }
}

impl App {
    pub fn fetch_status(&mut self) {
        let repo = match Repository::init("./") {
            Ok(repo) => repo,
            Err(e) => panic!("failed to init: {}", e),
        };

        println!("state: {:?}", repo.state());
        println!("path: {:?}", repo.path());

        if repo.is_bare() {
            panic!("bare repo")
        }

        let statuses = repo.statuses(None).unwrap();

        self.status_items = Vec::new();
        self.index_items = Vec::new();

        for e in statuses.iter() {
            let status: Status = e.status();
            if status.is_ignored() {
                continue;
            }

            if status.is_index_new() || status.is_index_modified() {
                self.index_items.push(format!(
                    "{} ({:?})",
                    e.path().unwrap().to_string(),
                    status
                ))
            }

            if status.is_wt_new() || status.is_wt_modified() {
                self.status_items.push(e.path().unwrap().to_string())
            }
        }

        self.status_select = if self.status_items.len() > 0 {
            Some(0)
        } else {
            None
        };

        self.update_diff();
        self.count += 1;
    }

    fn update_diff(&mut self) {
        let new_diff = match self.status_select {
            Some(i) => get_diff(Path::new(self.status_items[i].as_str())),
            None => Diff::default(),
        };

        if new_diff != self.diff {
            self.diff = new_diff;
            self.offset = 0;
        }
    }

    pub fn get_diff(&self) -> String {
        format!("{:#?}", self.diff.0)
    }
}

fn get_diff(p: &Path) -> Diff {
    let repo = Repository::init("./").unwrap();

    if repo.is_bare() {
        panic!("bare repo")
    }

    let diff = repo.diff_index_to_workdir(None, None).unwrap();

    let mut res = Vec::new();

    diff.print(DiffFormat::Patch, |delta, _hunk, line| {
        if p != delta.old_file().path().unwrap() {
            return true;
        }
        if p != delta.new_file().path().unwrap() {
            return true;
        }

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


#[cfg(test)]
mod tests {
    use  super::*;

    #[test]
    fn test_diff() {
        get_diff(Path::new("crates/base/src/app.rs"));
    }
}
