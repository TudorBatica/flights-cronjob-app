use crate::routes::trips;
use crate::{api, routes};
use actix_files::NamedFile;
use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> std::io::Result<Server> {
    println!("Starting flights-web {}...", listener.local_addr().unwrap());

    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(routes::health_check))
            .route("/new_trip", web::get().to(new_trip))
            .service(api::locations::fetch_cities)
            .service(api::locations::fetch_countries)
            .service(api::locations::fetch_subdivisions)
            .service(api::register_trip::register)
            .service(actix_files::Files::new("/static", "./static").show_files_listing())
            .route("/", web::get().to(trips))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn new_trip(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static/new_trip.html")?)
}
