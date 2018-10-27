#[macro_use]
extern crate wayland_protocol_docs_derive;
#[macro_use]
extern crate askama;
extern crate xmltree;
extern crate unindent;
extern crate glob;

use askama::Template;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use glob::glob;

mod protocol;
use protocol::Protocol;
mod filters;
mod format;

#[derive(Template)]
#[template(path = "protocol.html")]
struct ProtocolTemplate<'a> {
    protocol: &'a Protocol,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    protocols: Vec<ProtocolDetails>,
}

struct ProtocolDetails {
    pub url: String,
    pub name: String
}

fn main() -> std::io::Result<()> {
    let protocol_files = glob("./data/**/*.xml")
        .unwrap()
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let protocols = protocol_files
        .clone()
        .into_iter()
        .map(|p| p.to_str().unwrap().to_string())
        .map(|s| Protocol::from_file(&s));

    let mut details = vec![];

    fs::create_dir_all("site/protocols")?;

    for protocol in protocols {
        {
            let template = ProtocolTemplate{protocol: &protocol};
            let filename = format!("site/protocols/{}.html", protocol.name);
            render_to_file(&template, &filename)?;
        }

        let url = format!("./protocols/{}.html", protocol.name);
        details.push(ProtocolDetails{url: url, name: protocol.name});
    }

    let index_template = IndexTemplate{protocols: details};
    render_to_file(&index_template, "site/index.html")?;

    Ok(())
}

fn render_to_file<T: Template>(template: &T, file: &str) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write_fmt(format_args!("{}", template.render().unwrap()))?;
    Ok(())
}
