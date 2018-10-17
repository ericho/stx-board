extern crate actix_web;
extern crate handlebars;
extern crate storyboard_client;
extern crate serde_json;

use actix_web::{server, App, HttpRequest, HttpResponse};
use storyboard_client::{Client, ProjectGroup, Task};
use handlebars::Handlebars;


const STARLINGX_ID: i32 = 86;

struct AppState {
    client: Client,
    templates: Handlebars,
}

fn register_templates() -> Handlebars {
    let mut h = Handlebars::new();
    h.register_template_file("index", "./templates/index.hbs").unwrap();
    h.register_template_file("all_tasks", "./templates/all_tasks.hbs").unwrap();
    h
}

fn all_tasks(req: &HttpRequest<AppState>) -> Result<HttpResponse, std::io::Error> {

    let group: ProjectGroup = ProjectGroup { id: STARLINGX_ID, ..Default::default() };
    let tasks = req.state()
        .client
        .get_tasks_in_project_group(&group)
        .unwrap();

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


fn main() {
    server::new(
        || App::with_state(AppState {
            client: Client::new("https://storyboard.openstack.org/api/v1"),
            templates: register_templates(),
        }).resource("/", |r| r.get().f(all_tasks)))
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
}
