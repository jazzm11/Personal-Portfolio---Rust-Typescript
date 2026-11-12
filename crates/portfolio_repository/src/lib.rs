use portfolio_models::Project;

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Personal Portfolio".to_string(),
            description: "My personal portfolio built with Rust and TypeScript.".to_string(),
            technologies: vec![
                "Rust".to_string(),
                "Axum".to_string(),
                "Lit".to_string(),
                "TypeScript".to_string(),
            ],
            github_url: None,
            demo_url: None,
        },
    ]
}