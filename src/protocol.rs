use xmltree::Element;
use std::fs::File;

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

        let interfaces: Vec<Interface> = protocol
            .children
            .clone()
            .into_iter()
            .filter(|x| x.name == "interface")
            .map(|i| Interface::from_element(&i))
            .collect();
        let copyright = protocol.get_child("copyright").map(|e| e.text.as_ref().unwrap().to_string());
        Protocol {
            name,
            interfaces,
            copyright,
        }
    }
}

pub struct Interface {
    pub name: String,
    pub version: String,
    pub description: Option<Description>,
    pub requests: Vec<Request>,
}

impl Interface {
    pub fn from_element(element: &Element) -> Interface {
        let requests: Vec<Request> = element
            .children
            .clone()
            .into_iter()
            .filter(|x| x.name == "request")
            .map(|r| Request::from_element(&r))
            .collect();
        Interface {
            name: element.attributes["name"].to_string(),
            version: element.attributes["version"].to_string(),
            description: Description::from_parent(&element),
            requests,
        }
    }
}

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
