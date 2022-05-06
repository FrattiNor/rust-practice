use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 普通例子
fn example1() {
    // 创建通道
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // 发送值
        tx.send(String::from("aaa")).unwrap();
    });

    // 阻塞等待一个值
    let r = rx.recv().unwrap();
    println!("{}", r);
}

// 多发送者的例子
fn example2() {
    // 创建通道
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // 发送多个值
        for i in [1, 2, 3] {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 遍历直到通道关闭（子线程结束，tx被回收）
    for i in rx {
        println!("{}", i)
    }
}

// 多线程多发送者的例子（通过clone通道）
fn example3() {
    // 创建通道
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        // 发送多个值
        for i in [1, 2, 3] {
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        // 发送多个值
        for i in [4, 5, 6] {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 遍历直到通道关闭（子线程结束，tx被回收）
    for i in rx {
        println!("{}", i)
    }
}

fn main() {
    example3()
}
