use portfolio_models::Project;
pub mod database;

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            title: "ROS Agent".to_string(),
            description: "This  project is about an agent that helps you analyzes a project and makes a ROS-analize document.".to_string(),
            technologies: vec![
                "Copilot Studio".to_string(),
                "Power Apps".to_string(),
                "Copilot".to_string(),
                "Teams".to_string(),
            ],
            github_url: None,
            demo_url: None,
        },
        Project {
            title: "Portfolio".to_string(),
            description: "My personal portfolio about my skill, projects, experience and about me.".to_string(),
            technologies: vec![
                "Rust".to_string(),
                "Axum".to_string(),
                "SQLx".to_string(),
                "TypeScript".to_string(),
            ],
            github_url: None,
            demo_url: None,
        },
    ]
}