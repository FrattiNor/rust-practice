use std::process::exit;

mod lib;

fn main() {
    let config = lib::Config::new().unwrap_or_else(|e| {
        // cargo run > output.log
        // 标准输出到output.log中
        // eprintln打印标准错误，不会被打印到output.log中
        // 意为着stdout和stderr分开打印
        eprintln!("error: {}", e);
        exit(1);
    });

    println!("{:?}", config);

    config.run().unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        exit(1);
    });
}
