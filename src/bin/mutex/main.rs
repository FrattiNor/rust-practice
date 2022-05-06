use std::time::Duration;
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn example1() {
    // 创建线程锁
    let m = Mutex::new(0);

    {
        // 获取锁
        let mut m_lock = m.lock().unwrap();
        // 修改值
        *m_lock += 1;
        // 睡眠1s
        thread::sleep(Duration::from_secs(1));
    }

    // 获取并打印锁
    println!("{}", *m.lock().unwrap());
}

fn example2() {
    // 创建共享锁（Arc允许多个拥有者，和RC类似，但是是原子性的【线程安全】）
    let m = Arc::new(Mutex::new(0));
    // 创建容纳线程句柄的vec
    let mut h = vec![];

    for _ in 0..10 {
        // 克隆锁
        let m_clone = Arc::clone(&m);
        // 新建线程
        let handle = thread::spawn(move || {
            // 获取锁
            let mut m_lock = m_clone.lock().unwrap();
            // 修改值
            *m_lock += 1;
            // 睡眠1s
            thread::sleep(Duration::from_secs(1));
        });
        // 存储线程句柄
        h.push(handle);
    }

    for handle in h {
        // 等待线程结束
        handle.join().unwrap();
    }

    // 获取并打印锁
    println!("{}", *m.lock().unwrap());
}

fn main() {
    example2();
}
