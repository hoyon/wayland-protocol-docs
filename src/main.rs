#[macro_use]
extern crate wayland_protocol_docs_derive;
#[macro_use]
extern crate askama;
extern crate xmltree;

use askama::Template;
use std::fs::File;
use std::io::prelude::*;

mod protocol;
use protocol::Protocol;
mod filters;

#[derive(Template)]
#[template(path = "protocol.html")]
struct ProtocolTemplate<'a> {
    protocol: &'a Protocol,
}

fn main() -> std::io::Result<()> {
    let protocol = Protocol::from_file("./data/xdg-shell.xml");
    let template = ProtocolTemplate {
        protocol: &protocol
    };
    render_to_file(&template, "site/index.html")?;
    Ok(())
}

fn render_to_file<T: Template>(template: &T, file: &str) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write_fmt(format_args!("{}", template.render().unwrap()))?;
    Ok(())
}
