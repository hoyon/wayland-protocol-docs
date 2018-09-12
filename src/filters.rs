use askama::Result;
use protocol::Description;

pub fn unwrap(option: &Option<String>) -> Result<&str> {
    match option {
        Some(string) => Ok(string),
        None => Ok(""),
    }
}

pub fn desc_full(description: &Option<Description>) -> Result<&str> {
    match description {
        Some(Description { full, .. }) => Ok(full),
        None => Ok(""),
    }
}

pub fn desc_summary(description: &Option<Description>) -> Result<&str> {
    match description {
        Some(Description { summary, .. }) => Ok(summary),
        None => Ok(""),
    }
}
