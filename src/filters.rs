use askama::Result;
use protocol::{Describable, Interface, Request};
use format;

pub fn unwrap(option: &Option<String>) -> Result<&str> {
    match option {
        Some(string) => Ok(string),
        None => Ok(""),
    }
}

pub fn desc_full<D: Describable>(describable: &D) -> Result<String> {
    match describable.get_full() {
        Some(full) => Ok(full),
        None => Ok("".to_string()),
    }
}

pub fn desc_summary<D: Describable>(describable: &D) -> Result<String> {
    match describable.get_summary() {
        Some(summary) => Ok(summary),
        None => Ok("".to_string()),
    }
}

pub fn format_request(request: &Request, protocol_name: &str) -> Result<String> {
    Ok(format::format_request(&request, protocol_name))
}

pub fn interface_id(interface: &Interface) -> Result<String> {
    Ok(format!("i:{}", interface.name))
}

pub fn request_id(request: &Request, interface: &Interface) -> Result<String> {
    Ok(format!("r:{}:{}", interface.name, request.name))
}
