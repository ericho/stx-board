extern crate actix_web;
extern crate storyboard_client;

use actix_web::{server, App, HttpRequest, HttpResponse};
use storyboard_client::{Client, ProjectGroup};

// type StxResponse = Box<Future<Item = HttpResponse, Error = std::io::Error>>;

const STARLINGX_ID: i32 = 86;

struct AppState {
    client: Client,
}


fn all_tasks(req: &HttpRequest<AppState>) -> Result<HttpResponse, std::io::Error> {

    let group: ProjectGroup = ProjectGroup { id: STARLINGX_ID, ..Default::default() };
    let tasks = req.state().client.get_tasks_in_project_group(&group).unwrap();

    let body = tasks.iter().fold("".to_string(), |mut i, j| {
        let a = format!("{} {}<br/>", j.title.clone(),
                        j.status.clone().unwrap_or("none".to_string()));
        i.push_str(&a);
        i
    });

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}


fn main() {
    server::new(
        || App::with_state(AppState {
            client: Client::new("https://storyboard.openstack.org/api/v1"),
        }).resource("/", |r| r.get().f(all_tasks)))
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
}
