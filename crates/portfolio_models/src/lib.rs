use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub github_url: Option<String>,
    pub demo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub company: String,
    pub role: String,
    pub description: String,
    pub start_date: String,
    pub end_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub category: String,
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competency {
    pub title: String,
    pub description: String,
    pub status: String,
}