extern crate bigint;
use bigint::U512;
use std::io;

fn main() {
    let num: u16;
    println!("请输入n (u8), 计算对应的斐波那契值");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("标准输入错误");

        num = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("输入的不是数字");
                continue;
            }
        };

        break;
    }

    let mut res_list: Vec<U512> = vec![1.into(), 1.into()];

    for n in 2..num as usize {
        res_list.push(res_list[n - 2] + res_list[n - 1]);
    }

    println!(
        "{}对应的斐波那契值为：{}",
        num,
        res_list[res_list.len() - 1]
    )
}
