use git2::{DiffFormat, Repository, Status};

#[derive(Default)]
pub struct App {
    status_items: Vec<String>,
    status_select: Option<usize>,
    index_items: Vec<String>,
    diff: String,
    offset: u16,
    do_quit: bool,
}

impl App {
    pub fn is_quit(&self) -> bool {
        self.do_quit
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
                self.status_items.push(format!(
                    "{} ({:?})",
                    e.path().unwrap().to_string(),
                    status
                ))
            }
        }

        self.status_select = if self.status_items.len() > 0 {
            Some(0)
        } else {
            None
        };

        self.diff = self.get_diff();
    }

    pub fn get_diff(&mut self) -> String {
        let repo = Repository::init("./").unwrap();

        if repo.is_bare() {
            panic!("bare repo")
        }

        let diff = repo.diff_index_to_workdir(None, None).unwrap();

        let mut res = String::new();

        diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
            let content = String::from_utf8_lossy(line.content());
            res.push_str(content.chars().as_str());
            true
        })
        .unwrap();

        res
    }
}
