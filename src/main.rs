#[macro_use]
extern crate wayland_protocol_docs_derive;

use askama::Template;
use glob::glob;
use itertools::Itertools;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

mod protocol;
use crate::protocol::Protocol;
mod filters;
mod format;

#[derive(Template)]
#[template(path = "protocol.html")]
struct ProtocolTemplate<'a, 'b> {
    protocol: &'a Protocol,
    base_url: &'b str,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    grouped_protocols: HashMap<String, Vec<ProtocolDetails>>,
    base_url: &'a str,
}

struct ProtocolDetails {
    pub url: String,
    pub name: String,
    pub category: String,
}

fn main() -> std::io::Result<()> {
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "".to_owned());
    let protocols= glob("./data/**/*.xml")
        .unwrap()
        .filter_map(Result::ok)
        .map(|s| Protocol::from_file(&s));

    let mut details = vec![];

    fs::create_dir_all("site/protocols")?;

    for protocol in protocols {
        {
            let template = ProtocolTemplate {
                protocol: &protocol,
                base_url: &base_url,
            };
            let filename = format!("site/protocols/{}.html", protocol.name);
            render_to_file(&template, &filename)?;
        }

        let url = format!("{}/protocols/{}.html", base_url, protocol.name);
        details.push(ProtocolDetails {
            url,
            name: protocol.name,
            category: protocol.category,
        });
    }

    let mut grouped_protocols = HashMap::new();
    for (key, group) in &details.into_iter().group_by(|p| p.category.clone()) {
        grouped_protocols.insert(key, group.collect());
    }

    let index_template = IndexTemplate {
        grouped_protocols,
        base_url: &base_url,
    };
    render_to_file(&index_template, "site/index.html")?;

    Ok(())
}

fn render_to_file<T: Template>(template: &T, file: &str) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write_fmt(format_args!("{}", template.render().unwrap()))?;
    Ok(())
}
