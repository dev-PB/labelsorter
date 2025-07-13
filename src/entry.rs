use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub name: String,
    pub value: f64,
}
