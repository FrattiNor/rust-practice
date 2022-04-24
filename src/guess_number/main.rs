use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut small = 1;
    let mut big = 100;
    let n = rand::thread_rng().gen_range(small..=big);

    loop {
        println!("请输入猜的数, 最小{}，最大{}", small, big);

        let mut input = String::from("");
        io::stdin().read_line(&mut input).expect("标准输入错误");

        let input: u32 = match input.trim().parse() {
            Err(_) => {
                println!("输入的不是u32类型的数字");
                continue;
            }
            Ok(i) => i,
        };

        if input > big || input < small {
            println!("输入的数字不在范围内");
            continue;
        }

        match input.cmp(&n) {
            Ordering::Less => {
                println!("猜小了");
                small = input;
            }
            Ordering::Equal => {
                println!("恭喜你，猜对了！");
                break;
            }
            Ordering::Greater => {
                println!("猜大了");
                big = input;
            }
        }
    }
}
