use std::thread;
use std::time::Duration;
use base::APP;

#[test]
fn test_app_run() {
    base::init_app();

    let handle = thread::spawn(|| {
        base::init_app();
        let app = APP.read().unwrap();
        app.get_diff();
        app.log("thread");
        thread::sleep(Duration::from_secs(2));
    });

    thread::sleep(Duration::from_secs(1));
    base::init_app();
    println!("end");
    handle.join().unwrap();
}
