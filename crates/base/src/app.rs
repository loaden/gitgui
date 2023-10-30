use std::env;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::git_index::IndexComponent;
use crate::git_status::StatusLists;
use crate::git_utils::{self, Diff, DiffLine};

lazy_static! {
    pub static ref APP: RwLock<App> = RwLock::new(App::default());
}

#[derive(Default)]
pub struct App {
    repo: String,
    status: StatusLists,
    index: IndexComponent,
    diff: Diff,
    count: u32,
}

impl App {
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
    fn fetch_status(&mut self) {
        let new_status = StatusLists::from(self.repo_path());

        if self.status != new_status {
            self.status = new_status;
        }
    }

    fn update_diff(&mut self) {
        self.count += 1;

        let new_diff = match self.index.selection() {
            Some(i) => git_utils::get_diff(
                self.repo_path(),
                Path::new(i.path.as_str()),
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
        self.status.wt_items_pathlist()
    }

    pub fn get_index_items(&self) -> Vec<String> {
        self.status.index_items_pathlist()
    }

    pub fn set_status_select(&mut self, index: i32) {
        self.index.set_selection(index);
        self.update_diff();
    }

    pub fn update(&mut self) {
        self.fetch_status();
        self.index.update(self.repo_path());
        self.update_diff();
    }

    pub fn commit(&self, msg: String) -> bool {
        if self.status.index_items.is_empty() {
            return false;
        } else {
            git_utils::commit(self.repo_path(), msg);
            true
        }
    }

    pub fn index_add(&mut self) {
        if let Some(i) = self.index.selection() {
            if git_utils::index_add(self.repo_path(), i.path.clone()) {
                self.update();
            }
        }
    }
}
