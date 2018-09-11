#[macro_use]
extern crate askama;

use askama::Template;
use std::fs::File;
use std::io::prelude::*;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
    age: u32,
}

fn main() -> std::io::Result<()> {
    let hello = HelloTemplate { name: "bill" , age: 3};
    render_to_file(&hello, "site/index.html")?;
    Ok(())
}

fn render_to_file<T: Template>(template: &T, file: &str) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write_fmt(format_args!("{}", template.render().unwrap()))?;
    Ok(())
}
