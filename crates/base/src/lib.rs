pub mod app;

pub const G_APPNAME: &str = "GitGui";
pub const G_VERSION: &str = "0.0.1";

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn init_app() {
    let mut app = app::App::default();
    app.fetch_status();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
