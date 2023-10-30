use std::env;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use git2::StatusShow;
use lazy_static::lazy_static;

use crate::git_index::IndexComponent;
use crate::git_utils::{self, Diff, DiffLine};

lazy_static! {
    pub static ref APP: RwLock<App> = RwLock::new(App::new());
}

pub struct App {
    repo: String,
    index_wd: IndexComponent,
    index: IndexComponent,
    diff: Diff,
    count: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            repo: String::new(),
            diff: Diff::default(),
            index_wd: IndexComponent::new(StatusShow::Workdir, true),
            index: IndexComponent::new(StatusShow::Index, false),
            count: 0,
        }
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
        self.update();
    }

    pub fn repo_path(&self) -> String {
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
    fn update_diff(&mut self) {
        self.count += 1;

        let (idx, is_stage) = if self.index.focused() {
            (&self.index, true)
        } else {
            (&self.index_wd, false)
        };

        let new_diff = match idx.selection() {
            Some(i) => git_utils::get_diff(
                self.repo_path(),
                Path::new(i.path.as_str()),
                is_stage,
            ),
            None => Diff::default(),
        };

        if new_diff != self.diff {
            self.diff = new_diff;
        }
    }

    pub fn get_diff(&self) -> Vec<DiffLine> {
        self.diff.0.clone()
    }

    pub fn get_status_items(&self) -> Vec<String> {
        self.index_wd.get_items()
    }

    pub fn get_index_items(&self) -> Vec<String> {
        self.index.get_items()
    }

    pub fn set_selection(&mut self, index: i32, stage: bool) {
        self.index.focus(stage);
        self.index_wd.focus(!stage);
        let idx = if stage {
            &mut self.index
        } else {
            &mut self.index_wd
        };
        idx.set_selection(index);
        self.update_diff();
    }

    pub fn update(&mut self) {
        self.index.update(self.repo_path());
        self.index_wd.update(self.repo_path());
        self.update_diff();
    }

    pub fn commit(&self, msg: String) -> bool {
        if git_utils::index_empty(self.repo_path()) {
            return false;
        } else {
            git_utils::commit(self.repo_path(), msg);
            true
        }
    }

    pub fn index_add(&mut self) {
        if self.index_wd.focused() {
            if let Some(i) = self.index_wd.selection() {
                if git_utils::index_add(self.repo_path(), i.path.clone()) {
                    self.update();
                }
            }
        }
    }
}
