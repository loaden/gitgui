use std::thread;
use std::time::Duration;

pub use base::APP;

#[test]
fn test_app_run() {
    base::open_default_repo();

    let handle = thread::spawn(|| {
        base::open_default_repo();
        let app = APP.read().unwrap();
        app.get_diff();
        app.log("thread");
        thread::sleep(Duration::from_secs(2));
    });

    thread::sleep(Duration::from_secs(1));
    base::open_default_repo();
    println!("end");
    handle.join().unwrap();
}
