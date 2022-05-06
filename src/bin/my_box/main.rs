use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn t(a: String) {
    println!("{}", a);
}

fn main() {
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(x, *y);

    let z = String::from("_");
    t(z);
}
