#[derive(Debug)]
struct Man {}
trait A {}
trait B {}

impl A for Man {}

impl B for Man {}

fn main() {}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    #[test]
    fn test1() {
        assert_eq!(1, 1)
    }
    #[test]
    fn test2() {
        assert!(true)
    }
    #[test]
    fn test3() {
        assert_ne!(2, 1)
    }
    #[test]
    #[should_panic]
    fn test4() {
        assert_ne!(1, 1)
    }
    #[test]
    #[should_panic(expected = "ppp")]
    fn test5() {
        panic!("ppp");
    }
    #[test]
    fn test6() -> Result<(), anyhow::Error> {
        Ok(())
    }

    // ignore 的需要 cargo test -- --ignore进行测试
    #[test]
    #[ignore]
    fn test7() -> Result<(), anyhow::Error> {
        Ok(())
    }
}
