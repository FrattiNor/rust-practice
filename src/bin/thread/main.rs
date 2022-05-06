use std::thread;
use std::time::Duration;

fn main() {
    // 启用一个线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 主线程
    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等待线程结束，如果不等待，主线程结束了将杀死子线程
    handle.join().unwrap();
}
