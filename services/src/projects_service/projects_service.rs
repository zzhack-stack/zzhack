use crate::post_service::post_service::{Post, POST_SERVICE};
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Clone)]
pub struct RawProject {
    pub name: String,
    pub desc: String,
    pub addr: String,
    pub post: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct Project {
    pub name: String,
    pub desc: String,
    pub addr: String,
    pub post: Option<Post>,
}

#[derive(Deserialize, Clone)]
pub struct ProjectsData {
    pub projects: Vec<RawProject>,
}

pub struct ProjectsService {
    projects: Vec<Project>,
}

impl ProjectsService {
    pub fn new() -> ProjectsService {
        let projects_data = include_str!("./projects.json");
        let projects_data: ProjectsData = serde_json::from_str(projects_data).unwrap();
        let projects = projects_data
            .projects
            .iter()
            .map(|raw_project| {
                let post = match raw_project.post.clone() {
                    Some(filename) => POST_SERVICE.find_post_by_filename(&filename).clone(),
                    None => None,
                };

                Project {
                    addr: raw_project.addr.clone(),
                    name: raw_project.name.clone(),
                    desc: raw_project.desc.clone(),
                    post,
                }
            })
            .collect();

        ProjectsService { projects }
    }

    pub fn get_projects(&self) -> Vec<Project> {
        self.projects.clone()
    }

    pub fn get_projects_by_odd_even(&self) -> (Vec<Project>, Vec<Project>) {
        let mut even = vec![];
        let mut odd = vec![];

        for (i, project) in self.projects.iter().enumerate() {
            if (i + 1) % 2 == 0 {
                even.push(project.clone());
            } else {
                odd.push(project.clone());
            }
        }

        (odd, even)
    }
}

pub static PROJECTS_SERVICE: Lazy<ProjectsService> = Lazy::new(|| ProjectsService::new());
