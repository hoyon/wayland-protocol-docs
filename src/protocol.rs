use std::fs::File;
use xmltree::Element;

pub struct Protocol {
    pub name: String,
    pub interfaces: Vec<Interface>,
    pub copyright: Option<String>,
}

impl Protocol {
    pub fn from_file(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let protocol = Element::parse(file).unwrap();

        let name = protocol.attributes["name"].to_string();
        let interfaces = element_map(&protocol, "interface", |i| Interface::from_element(&i));
        let copyright = protocol
            .get_child("copyright")
            .map(|e| e.text.as_ref().unwrap().to_string());
        Protocol {
            name,
            interfaces,
            copyright,
        }
    }
}

#[derive(Describable)]
pub struct Interface {
    pub name: String,
    pub version: String,
    pub description: Option<Description>,
    pub requests: Vec<Request>,
}

impl Interface {
    pub fn from_element(element: &Element) -> Interface {
        let requests = element_map(element, "request", |r| Request::from_element(&r));
        Interface {
            name: element.attributes["name"].to_string(),
            version: element.attributes["version"].to_string(),
            description: Description::from_parent(&element),
            requests,
        }
    }
}

#[derive(Describable)]
pub struct Request {
    pub name: String,
    pub request_type: Option<String>,
    pub since: Option<String>,
    pub description: Option<Description>,
}

impl Request {
    pub fn from_element(element: &Element) -> Request {
        Request {
            name: element.attributes["name"].to_string(),
            request_type: element.attributes.get("type").map(|s| s.to_string()),
            since: element.attributes.get("since").map(|s| s.to_string()),
            description: Description::from_parent(&element),
        }
    }
}

pub trait Describable {
    fn get_full(&self) -> Option<String>;
    fn get_summary(&self) -> Option<String>;
}

#[derive(Clone)]
pub struct Description {
    pub full: String,
    pub summary: String,
}

impl Description {
    pub fn from_parent(parent: &Element) -> Option<Description> {
        parent.get_child("description").map(|element| Description {
            full: element.text.as_ref().unwrap().to_string(),
            summary: element.attributes["summary"].to_string(),
        })
    }
}

fn element_map<T, F>(element: &Element, name: &str, f: F) -> Vec<T>
where
    F: FnMut(Element) -> T,
{
    element
        .children
        .clone()
        .into_iter()
        .filter(|x| x.name == name)
        .map(f)
        .collect()
}
