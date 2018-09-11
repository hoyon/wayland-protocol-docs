#[macro_use]
extern crate askama;
extern crate xmltree;

use askama::Template;
use std::fs::File;
use std::io::prelude::*;
use xmltree::Element;

struct Description {
    full: String,
    summary: String,
}

impl Description {
    fn from_parent(parent: &Element) -> Self {
        let element = parent.get_child("description").unwrap();
        Description {
            full: element.text.as_ref().unwrap().to_string(),
            summary: element.attributes["summary"].to_string(),
        }
    }
}

struct Interface {
    name: String,
    description: Description,
}

#[derive(Template)]
#[template(path = "protocol.html")]
struct ProtocolTemplate<'a> {
    interfaces: &'a Vec<Interface>,
}

fn main() -> std::io::Result<()> {
    let file = File::open("./data/xdg-shell.xml")?;
    let protocol = Element::parse(file).unwrap();
    let interfaces: Vec<Interface> = protocol
        .children
        .into_iter()
        .filter(|x| x.name == "interface")
        .map(|i| Interface {
            name: i.attributes["name"].to_string(),
            description: Description::from_parent(&i),
        })
        .collect();

    let template = ProtocolTemplate {
        interfaces: &interfaces,
    };
    render_to_file(&template, "site/index.html")?;
    Ok(())
}

fn render_to_file<T: Template>(template: &T, file: &str) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write_fmt(format_args!("{}", template.render().unwrap()))?;
    Ok(())
}
