use std::env::{args, var};
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub search_text: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new() -> Result<Self, &'static str> {
        let args: Vec<String> = args().collect();

        let search_text = match args.get(1) {
            None => {
                return Err("缺少参数");
            }
            Some(t) => t,
        };

        let filename = match args.get(2) {
            None => {
                return Err("缺少参数");
            }
            Some(f) => f,
        };

        let un_case_insensitive = var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            search_text: search_text.to_string(),
            filename: filename.to_string(),
            case_insensitive: !un_case_insensitive,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let context = fs::read_to_string(&self.filename)?;

        let searched_line = if self.case_insensitive {
            search_case_insensitive(&self.search_text, &context)
        } else {
            search(&self.search_text, &context)
        };

        for line in searched_line {
            println!("{}", line);
        }

        Ok(())
    }
}

fn search_case_insensitive<'a>(search_text: &str, context: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    let search_text_lowercase = search_text.to_lowercase();

    for line in context.lines() {
        if line.to_lowercase().contains(&search_text_lowercase) {
            res.push(line)
        }
    }

    res
}

fn search<'a>(search_text: &str, context: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    for line in context.lines() {
        if line.contains(search_text) {
            res.push(line)
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_const() -> (
        &'static str,
        &'static str,
        Vec<&'static str>,
        Vec<&'static str>,
    ) {
        let search_text = "duct";
        let search_context = "Rust:\nsafe, fast, productive.\nPick three.\nDuct okk\n";
        let search_answer = vec!["safe, fast, productive."];
        let search_case_insensitive_answer = vec!["safe, fast, productive.", "Duct okk"];

        return (
            search_text,
            search_context,
            search_answer,
            search_case_insensitive_answer,
        );
    }

    #[test]
    fn test_search() {
        let (text, context, answer, _) = get_test_const();
        assert_eq!(answer, search(text, context));
    }

    #[test]
    fn test_search_case_insensitive() {
        let (text, context, _, answer) = get_test_const();
        assert_eq!(answer, search_case_insensitive(text, context));
    }
}
