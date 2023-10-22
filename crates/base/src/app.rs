use git2::{DiffFormat, Repository, Status};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

lazy_static::lazy_static! {
    pub static ref APP: RwLock<App> = RwLock::new(App::default());
}

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
pub struct Diff(Vec<DiffLine>);

#[derive(Default)]
pub struct App {
    repo: String,
    status_items: Vec<String>,
    status_select: Option<usize>,
    index_items: Vec<String>,
    diff: Diff,
    do_quit: bool,
    count: u32,
}

impl App {
    pub fn is_quit(&self) -> bool {
        self.do_quit
    }

    pub fn log(&self, msg: &str) {
        println!("COUNT: {}, MESSAGE: {}", self.count, msg);
    }

    pub fn src_path() -> PathBuf {
        let mut dir = env::current_exe().unwrap();
        loop {
            dir.push(".git");
            if dir.exists() {
                dir.pop(); // .git
                break;
            }
            dir.pop(); // .git
            dir.pop();
            if dir.parent().is_none() {
                break;
            }
        }
        dir
    }

    pub fn set_repo(&mut self, path: String) {
        self.repo = path;
        self.fetch_status();
    }

    pub fn get_repo(&self) -> String {
        if self.repo.is_empty() {
            String::from("./")
        } else {
            self.repo.clone()
        }
    }

    pub fn get_default_repo(&self) -> String {
        let repo = match App::src_path().to_str() {
            Some(src) => src.to_string(),
            None => String::new(),
        };
        repo
    }
}

impl App {
    pub fn fetch_status(&mut self) {
        let repo = match Repository::init(self.get_repo()) {
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
    }

    fn update_diff(&mut self) {
        self.count += 1;

        let new_diff = match self.status_select {
            Some(i) => {
                let path = Path::new(self.status_items[i].as_str());
                self.get_current_diff(path)
            }
            None => Diff::default(),
        };

        if new_diff != self.diff {
            self.diff = new_diff;
        }
    }

    pub fn get_diff(&self) -> Vec<DiffLine> {
        self.diff.0.clone()
    }

    fn get_current_diff(&self, p: &Path) -> Diff {
        let repo = Repository::init(self.get_repo()).unwrap();
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
}
