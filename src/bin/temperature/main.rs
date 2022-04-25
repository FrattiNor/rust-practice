use std::io;

fn main() {
    let mut type_num: u8;

    loop {
        let mut input = String::new();
        println!("请输入要选择的温度类型\n1.华氏温度\n2.摄氏度");
        io::stdin().read_line(&mut input).expect("标准输入错误");

        type_num = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("输入不正确");
                continue;
            }
        };

        match type_num {
            1 | 2 => {}
            _ => {
                println!("输入不正确");
                continue;
            }
        }

        break;
    }

    let temperature: f64;

    loop {
        let mut input = String::new();
        println!("请输入温度");
        io::stdin().read_line(&mut input).expect("标准输入错误");

        temperature = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("输入不正确");
                continue;
            }
        };

        break;
    }

    match type_num {
        1 => {
            println!(
                "华氏温度{}对应的摄氏度为：{}",
                temperature,
                (temperature - 32_f64) / 1.8
            );
        }
        2 => {
            println!(
                "摄氏度{}对应的华氏温度为：{}",
                temperature,
                temperature * 1.8 + 32_f64
            );
        }
        _ => {}
    }
}
