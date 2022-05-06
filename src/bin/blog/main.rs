mod blog;

fn main() {
    let mut p = blog::Blog::new();

    p.add_text("okk okk").unwrap();

    p.request_review().unwrap();

    p.approve().unwrap();

    p.add_text("zzz").unwrap();

    p.request_review().unwrap();

    p.approve().unwrap();

    println!("{}", p);
}
