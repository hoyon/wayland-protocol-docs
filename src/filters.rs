use askama::Result;
use protocol::Describable;

pub fn unwrap(option: &Option<String>) -> Result<&str> {
    match option {
        Some(string) => Ok(string),
        None => Ok(""),
    }
}

pub fn desc_full<'a, D: Describable>(describable: &'a D) -> Result<String> {
    match describable.get_full() {
        Some(full) => Ok(full),
        None => Ok("".to_string()),
    }
}

pub fn desc_summary<'a, D: Describable>(describable: &'a D) -> Result<String> {
    match describable.get_summary() {
        Some(summary) => Ok(summary),
        None => Ok("".to_string()),
    }
}
