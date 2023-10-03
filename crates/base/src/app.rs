use git2::Repository;

#[derive(Default)]
pub struct App {
    status_items: Vec<String>,
    status_select: Option<usize>,
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

        println!("state: {:?}",repo.state());
        println!("path: {:?}",repo.path());

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
    }
}
