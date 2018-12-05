extern crate actix_web;
extern crate handlebars;
extern crate storyboard_client;
extern crate serde_json;

use actix_web::{
    fs,
    server, App, HttpRequest, HttpResponse
};
use storyboard_client::{Client, ProjectGroup, Task};
use handlebars::Handlebars;

// Temp use for testing
use std::fs::File;
use std::io::Read;


const STARLINGX_ID: i32 = 86;

struct AppState {
    client: Client,
    templates: Handlebars,
}

fn register_templates() -> Handlebars {
    let mut h = Handlebars::new();
    h.register_template_file("index", "./src/templates/index.hbs").unwrap();
    h.register_template_file("all_tasks", "./src/templates/all_tasks.hbs").unwrap();
    h
}

fn all_tasks(req: &HttpRequest<AppState>) -> Result<HttpResponse, std::io::Error> {

    let group: ProjectGroup = ProjectGroup { id: STARLINGX_ID, ..Default::default() };
    //let _tasks = req.state()
    //    .client
    //    .get_tasks_in_project_group(&group)
    //    .unwrap();

    let mut file = File::open("tests/tasks.json")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let tasks: Vec<Task> = serde_json::from_str(&s)?;

    let tasks: Vec<Task> = tasks.into_iter()
        .filter(|t|
                // Remove all the merged and invalid tasks from the list
                if let Some(ref a) = t.status {
                    if a.contains("merged") || a.contains("invalid") {
                        false
                    } else {
                        true
                    }
                } else {
                    false
                })
        .collect();

    let body = req.state()
        .templates
        .render("all_tasks", &tasks)
        .unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[derive(Default)]
struct StaticFileConfig;

impl fs::StaticFileConfig for StaticFileConfig {
    fn is_use_etag() -> bool {
        true
    }
}

fn main() {
    server::new(
        || App::with_state(AppState {
            client: Client::new("https://storyboard.openstack.org/api/v1"),
            templates: register_templates(),
        }).handler(
            "/static",
            fs::StaticFiles::with_config("src/dist", StaticFileConfig).unwrap()
        ).resource("/", |r| r.get().f(all_tasks)))
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
}
