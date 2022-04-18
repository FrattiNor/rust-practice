#![forbid(unsafe_code)]

use anyhow::Result;
use clap::{Arg, Command};
// use std::collections::HashMap;

fn app<'a>() -> Command<'a> {
    return Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("input")
                .help("translate word or statement")
                .index(1),
        )
        .arg(
            Arg::new("type")
                .help("the translate api: google microsoft baidu")
                .short('T')
                .long("type")
                .default_value("google"),
        );
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = app().get_matches();

    match app.value_of("input") {
        Some(input) => println!("{}", input),
        None => println!("no input"),
    }
    match app.value_of("type") {
        Some(app_type) => println!("{}", app_type),
        None => println!("no type"),
    }

    // let res = reqwest::get(
    //     "https://api.juejin.cn/user_api/v1/user/get?aid=2608&uuid=7086012728590812683&not_self=0",
    // )
    // .await?;
    // .json::<HashMap<String, String>>()
    // .await?;

    let res = reqwest::get(
        "https://translate.google.cn/translate_a/single?client=at&sl=en&tl=zh-CN&dt=t&q=google",
    )
    .await?;

    println!("{:#?}", res);

    Ok(())
}
