use actix_web::{HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "trips.html")]
struct TripsTemplate {
    routes_count: i32,
    user_name: String,
}

pub async fn trips() -> impl Responder {
    let template = TripsTemplate {
        routes_count: 10,
        user_name: "Tudor".parse().unwrap(),
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}
