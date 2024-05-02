use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use flights_data::db_schema::{MonitoredRoutes, Trip, Trips};
use sea_query::{Expr, JoinType, PostgresQueryBuilder, Query};
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "trips.html")]
struct TripsTemplate {
    user_name: String,
    trips: Vec<Trip>,
}

pub async fn trips(pool: web::Data<PgPool>) -> impl Responder {
    let trips_query = Query::select()
        .expr(Expr::cust("*"))
        .from(Trips::Table)
        .join(
            JoinType::InnerJoin,
            MonitoredRoutes::Table,
            Expr::col((Trips::Table, Trips::AirportCode))
                .equals((MonitoredRoutes::Table, MonitoredRoutes::AirportCode))
                .and(
                    Expr::col((Trips::Table, Trips::CountryCode))
                        .equals((MonitoredRoutes::Table, MonitoredRoutes::CountryCode)),
                )
                .and(
                    Expr::col((Trips::Table, Trips::Price))
                        .lte(Expr::col((MonitoredRoutes::Table, MonitoredRoutes::Budget))),
                ),
        )
        .and_where(Expr::col(MonitoredRoutes::MonitoredBy).eq(1))
        .to_string(PostgresQueryBuilder);

    let trips: Vec<Trip> = sqlx::query_as(&trips_query)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    let template = TripsTemplate {
        trips,
        user_name: "Tudor".parse().unwrap(),
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}
