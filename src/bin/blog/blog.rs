use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Blog {
    state: i8,
    text: String,
    content: Vec<String>,
}

impl Blog {
    pub fn new() -> Self {
        Blog {
            state: 0,
            text: String::from(""),
            content: vec![],
        }
    }

    pub fn add_text(&mut self, str: &str) -> Result<(), &str> {
        if self.state == 0 {
            self.text = str.to_string();
            self.state = 1;
        } else {
            return Err("状态不正确");
        }

        Ok(())
    }

    pub fn request_review(&mut self) -> Result<(), &str> {
        if self.state == 1 {
            if self.text.len() <= 10 {
                self.state = 2;
            } else {
                return Err("审核不通过");
            }
        } else {
            return Err("状态不正确");
        }

        Ok(())
    }

    pub fn approve(&mut self) -> Result<(), &str> {
        if self.state == 2 {
            self.content.push(self.text.clone());
            self.state = 0;
            self.text.clear();
        } else {
            return Err("状态不正确");
        }

        Ok(())
    }
}

impl Display for Blog {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for t in self.content.iter() {
            res.push_str(&format!("{}\n", t));
        }

        write!(f, "{}", res)
    }
}
