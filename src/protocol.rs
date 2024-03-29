use std::fs::File;
use std::path::Path;
use xmltree::Element;

pub struct Protocol {
    pub name: String,
    pub interfaces: Vec<Interface>,
    pub copyright: Option<String>,
    pub category: String,
}

impl Protocol {
    pub fn from_file(path: &Path) -> Self {
        let file = File::open(path.to_str().unwrap().to_string()).unwrap();
        let protocol = Element::parse(file).unwrap();

        let name = protocol.attributes["name"].to_string();
        let interfaces = element_map(&protocol, "interface", |i| Interface::from_element(&i));
        let copyright = protocol
            .get_child("copyright")
            .map(|e| e.get_text().unwrap().to_string())
            .map(unindent_string);

        let category = path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        Protocol {
            name,
            interfaces,
            copyright,
            category,
        }
    }
}

#[derive(Describable)]
pub struct Interface {
    pub name: String,
    pub version: String,
    pub description: Option<Description>,
    pub requests: Vec<Request>,
    pub events: Vec<Event>,
    pub enums: Vec<Enum>,
}

impl Interface {
    pub fn from_element(element: &Element) -> Interface {
        let requests = element_map(element, "request", |r| Request::from_element(&r));
        let events = element_map(element, "event", |e| Event::from_element(&e));
        let enums = element_map(element, "enum", |e| Enum::from_element(&e));
        Interface {
            name: element.attributes["name"].to_string(),
            version: element.attributes["version"].to_string(),
            description: Description::from_parent(&element),
            requests,
            events,
            enums,
        }
    }
}

#[derive(Describable)]
pub struct Request {
    pub name: String,
    pub request_type: Option<String>,
    pub since: Option<String>,
    pub description: Option<Description>,
    pub args: Vec<Arg>,
}

impl Request {
    pub fn from_element(element: &Element) -> Request {
        let mut args = element_map(element, "arg", |a| Arg::from_element(&a));

        // Magical extra arguments for wl_registry_bind
        if args
            .iter()
            .any(|a| a.arg_type == "new_id" && a.interface == None)
        {
            args.push(Arg {
                name: "interface".to_string(),
                arg_type: "object".to_string(),
                summary: None,
                interface: Some("wl_interface".to_string()),
                allow_null: None,
                enum_name: None,
                description: None,
            });

            args.push(Arg {
                name: "version".to_string(),
                arg_type: "uint".to_string(),
                summary: None,
                interface: None,
                allow_null: None,
                enum_name: None,
                description: None,
            });
        }

        Request {
            name: get_attribute(element, "name"),
            request_type: get_optional_attribute(element, "type"),
            since: get_optional_attribute(element, "since"),
            description: Description::from_parent(&element),
            args,
        }
    }
}

#[derive(Describable)]
pub struct Event {
    pub name: String,
    pub since: Option<String>,
    pub description: Option<Description>,
    pub args: Vec<Arg>,
}

impl Event {
    pub fn from_element(element: &Element) -> Event {
        let args = element_map(element, "arg", |a| Arg::from_element(&a));
        Event {
            name: get_attribute(element, "name"),
            since: get_optional_attribute(element, "since"),
            description: Description::from_parent(&element),
            args,
        }
    }
}

#[derive(Describable)]
pub struct Enum {
    pub name: String,
    pub since: Option<String>,
    pub bitfield: Option<String>,
    pub description: Option<Description>,
    pub entries: Vec<EnumEntry>,
}

impl Enum {
    pub fn from_element(element: &Element) -> Enum {
        let entries = element_map(element, "entry", |e| EnumEntry::from_element(&e));
        Enum {
            name: get_attribute(element, "name"),
            since: get_optional_attribute(element, "since"),
            bitfield: get_optional_attribute(element, "bitfield"),
            description: Description::from_parent(&element),
            entries,
        }
    }
}

#[derive(Describable)]
pub struct EnumEntry {
    pub name: String,
    pub value: String,
    pub summary: Option<String>,
    pub since: Option<String>,
    pub description: Option<Description>,
}

impl EnumEntry {
    pub fn from_element(element: &Element) -> EnumEntry {
        EnumEntry {
            name: get_attribute(element, "name"),
            value: get_attribute(element, "value"),
            summary: get_optional_attribute(element, "summary"),
            since: get_optional_attribute(element, "since"),
            description: Description::from_parent(&element),
        }
    }
}

#[derive(Describable)]
pub struct Arg {
    pub name: String,
    pub arg_type: String,
    pub summary: Option<String>,
    pub interface: Option<String>,
    pub allow_null: Option<String>,
    pub enum_name: Option<String>,
    pub description: Option<Description>,
}

impl Arg {
    pub fn from_element(element: &Element) -> Arg {
        Arg {
            name: get_attribute(element, "name"),
            arg_type: get_attribute(element, "type"),
            summary: get_optional_attribute(element, "summary"),
            interface: get_optional_attribute(element, "interface"),
            allow_null: get_optional_attribute(element, "allow-null"),
            enum_name: get_optional_attribute(element, "enum"),
            description: Description::from_parent(element),
        }
    }
}

pub trait Describable {
    fn get_full(&self) -> Option<String>;
    fn get_summary(&self) -> Option<String>;
}

#[derive(Clone)]
pub struct Description {
    pub full: Option<String>,
    pub summary: String,
}

impl Description {
    pub fn from_parent(parent: &Element) -> Option<Description> {
        parent.get_child("description").map(|element| Description {
            full: element.get_text().map(|s| unindent_string(s.to_string())),
            summary: unindent_string(element.attributes["summary"].to_string()),
        })
    }
}

fn element_map<T, F>(element: &Element, name: &str, f: F) -> Vec<T>
where
    F: FnMut(&Element) -> T,
{
    element
        .children
        .iter()
        .filter_map(|node| node.as_element())
        .filter(|e| e.name == name)
        .map(f)
        .collect()
}

fn get_attribute(element: &Element, attribute: &str) -> String {
    element.attributes[attribute].to_string()
}

fn get_optional_attribute(element: &Element, attribute: &str) -> Option<String> {
    element.attributes.get(attribute).map(|s| s.to_string())
}

fn unindent_string(s: String) -> String {
    let replaced = s.replace("\t", "    ");
    unindent::unindent(&replaced)
}
