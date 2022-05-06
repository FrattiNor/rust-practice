// 获取item的不可变引用
fn use_iter() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let mut sum = 0;

    for item in v1_iter {
        // item 为 &i32
        sum += item;
    }

    println!("{}", sum);
}

// 获取item的可变引用
fn use_iter_mut() {
    let mut v1 = vec![String::from("1"), String::from("2"), String::from("3")];

    let v1_iter = v1.iter_mut();

    let mut sum = String::from("");

    for item in v1_iter {
        // item 为 &mut String
        sum += item;
    }

    println!("{}", sum);
}

// 获取item的获取所有权
fn use_into_iter() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.into_iter();

    let mut sum = 0;

    for mut item in v1_iter {
        // item 为 i32
        item = item + 1;
        sum += item;
    }

    println!("{}", sum);
}

fn main() {
    use_iter();
    use_iter_mut();
    use_into_iter();
}
