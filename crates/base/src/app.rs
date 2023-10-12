use git2::{DiffFormat, Repository};

#[derive(Default)]
pub struct App {
    status_items: Vec<String>,
    status_select: Option<usize>,
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

        let status = repo.statuses(None).unwrap();

        self.status_items = status
            .iter()
            .map(|e| e.path().unwrap().to_string())
            .collect();

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
