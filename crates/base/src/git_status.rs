use crate::git_utils;
use git2::Repository;
use git2::{Status, StatusOptions, StatusShow};

#[derive(PartialEq)]
pub enum StatusItemType {
    New,
    Modified,
    Deleted,
    Renamed,
    Typechange,
}

impl From<Status> for StatusItemType {
    fn from(s: Status) -> Self {
        if s.is_index_new() || s.is_wt_new() {
            Self::New
        } else if s.is_index_deleted() || s.is_wt_deleted() {
            Self::Deleted
        } else if s.is_index_renamed() || s.is_wt_renamed() {
            Self::Renamed
        } else if s.is_index_typechange() || s.is_wt_typechange() {
            Self::Typechange
        } else {
            Self::Modified
        }
    }
}

#[derive(Default, PartialEq)]
pub struct StatusItem {
    pub path: String,
    pub status: Option<StatusItemType>,
}

#[derive(Default, PartialEq)]
pub struct StatusLists {
    pub wt_items: Vec<StatusItem>,
    pub index_items: Vec<StatusItem>,
}

impl StatusLists {
    pub fn from(path: String) -> Self {
        let mut res = Self::default();

        let repo = git_utils::repo(path);

        res.wt_items = Self::get(&repo, StatusShow::Workdir);
        res.index_items = Self::get(&repo, StatusShow::Index);

        res
    }

    fn get(repo: &Repository, show: StatusShow) -> Vec<StatusItem> {
        let mut res = Vec::new();

        let statuses = repo
            .statuses(Some(
                StatusOptions::default()
                    .show(show)
                    .include_untracked(true)
                    .renames_head_to_index(true)
                    .recurse_untracked_dirs(true),
            ))
            .unwrap();

        for e in statuses.iter() {
            let status: Status = e.status();
            let path = if let Some(diff) = e.head_to_index() {
                String::from(diff.new_file().path().unwrap().to_str().unwrap())
            } else {
                e.path().unwrap().to_string()
            };

            res.push(StatusItem {
                path,
                status: Some(StatusItemType::from(status)),
            });
        }

        res
    }

    pub fn wt_items_pathlist(&self) -> Vec<String> {
        self.wt_items.iter().map(|e| e.path.clone()).collect()
    }

    pub fn index_items_pathlist(&self) -> Vec<String> {
        self.index_items.iter().map(|e| e.path.clone()).collect()
    }
}
