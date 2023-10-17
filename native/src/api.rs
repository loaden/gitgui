use base;

pub fn app_run() {
    base::init_app();
}

pub fn get_diff() -> String {
    let app = base::APP.read().unwrap();
    app.get_diff()
}
