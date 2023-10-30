use crate::git_status::{self, StatusItem};
use git2::StatusShow;

pub struct IndexComponent {
    items: Vec<StatusItem>,
    index_type: StatusShow,
    selection: Option<usize>,
    focused: bool,
}

impl IndexComponent {
    pub fn new(index_type: StatusShow, focus: bool) -> Self {
        Self {
            items: Vec::new(),
            index_type,
            selection: None,
            focused: focus,
        }
    }

    pub fn update(&mut self, repo: String) {
        let new_status = git_status::get_index(&repo, self.index_type);

        if self.items != new_status {
            self.items = new_status;

            self.selection = if self.items.len() > 0 { Some(0) } else { None };
        }
    }

    pub fn get_items(&self) -> Vec<String> {
        self.items
            .iter()
            .map(|e| e.path.clone())
            .collect::<Vec<_>>()
    }

    pub fn selection(&self) -> Option<StatusItem> {
        match self.selection {
            None => None,
            Some(i) => Some(self.items[i].clone()),
        }
    }

    pub fn set_selection(&mut self, delta: i32) {
        let items_len = self.items.len();
        if items_len > 0 && delta < items_len as i32 {
            self.selection = Some(delta as usize);
        }
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn focus(&mut self, focus: bool) {
        self.focused = focus
    }
}
