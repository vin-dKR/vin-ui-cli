use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentConfig {
    pub name: String,
    pub dependencies: Option<Vec<String>>,
    pub utils: Option<Vec<String>>,
}
