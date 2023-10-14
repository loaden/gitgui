pub mod app;
pub use app::{App, APP};

pub fn init_app() {
    let cnt;
    unsafe {
        app::COUNT += 1;
        cnt = app::COUNT;
    }

    let app = App::global();
    app.log(format!("app.log -> COUNT: {}", cnt));
    app.get_diff();
    unsafe {
        let app = APP.get_mut().unwrap();
        app.fetch_status();
    }
}

#[cfg(test)]
mod tests {}
