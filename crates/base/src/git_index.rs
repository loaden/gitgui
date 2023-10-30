use crate::git_status::{self, StatusItem};
use git2::StatusShow;
use std::cmp;

pub struct IndexComponent {
    items: Vec<StatusItem>,
    index_type: StatusShow,
    selection: Option<usize>,
}

impl Default for IndexComponent {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            index_type: StatusShow::Workdir,
            selection: None,
        }
    }
}

impl IndexComponent {
    pub fn update(&mut self, repo: String) {
        let new_status = git_status::get_index(&repo, self.index_type);

        if self.items != new_status {
            self.items = new_status;

            self.selection = if self.items.len() > 0 { Some(0) } else { None };
        }
    }

    pub fn selection(&self) -> Option<StatusItem> {
        match self.selection {
            None => None,
            Some(i) => Some(self.items[i].clone()),
        }
    }

    pub fn set_selection(&mut self, delta: i32) {
        let items_len = self.items.len();
        if items_len > 0 && delta < items_len as i32  {
            self.selection = Some(delta as usize);
        }
    }

    pub fn move_selection(&mut self, delta: i32) {
        let items_len = self.items.len();
        if items_len > 0 {
            if let Some(i) = self.selection {
                let mut i = i as i32;

                i = cmp::min(i + delta, (items_len - 1) as i32);
                i = cmp::max(i, 0);

                self.selection = Some(i as usize);
            }
        }
    }
}
