use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentConfig {
    pub name: Option<String>,
    pub description: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub utils: Option<Vec<String>>,
    pub additional_files: Option<Vec<AdditionalFile>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdditionalFile {
    pub source: String,           // Source file path in templates/utils
    pub dest: Option<String>,     // Optional custom destination path (relative to appropriate dir)
    pub description: Option<String>, // Optional description
}
