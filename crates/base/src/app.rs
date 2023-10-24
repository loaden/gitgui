use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::{env, usize};

use lazy_static::lazy_static;

use crate::git_status::StatusLists;
use crate::git_utils::{self, Diff, DiffLine};

lazy_static! {
    pub static ref APP: RwLock<App> = RwLock::new(App::default());
}

#[derive(Default)]
pub struct App {
    repo: String,
    status: StatusLists,
    status_select: Option<usize>,
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
        let new_status = StatusLists::from(self.get_repo());

        if self.status != new_status {
            self.status = new_status;

            self.status_select = if self.status.wt_items.len() > 0 {
                Some(0)
            } else {
                None
            };
        }

        self.update_diff();
    }

    fn update_diff(&mut self) {
        self.count += 1;

        let new_diff = match self.status_select {
            Some(i) => {
                let path = Path::new(self.status.wt_items[i].path.as_str());
                git_utils::get_diff(self.get_repo(), path)
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

    pub fn get_status_items(&self) -> Vec<String> {
        self.status.wt_items_pathlist()
    }

    pub fn get_index_items(&self) -> Vec<String> {
        self.status.index_items_pathlist()
    }

    pub fn set_status_select(&mut self, index: usize) {
        self.status_select = Some(index);
        self.update_diff();
    }
}
