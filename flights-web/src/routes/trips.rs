use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use sea_query::{Alias, Expr, Func, JoinType, PostgresQueryBuilder, Query};
use sqlx::PgPool;

use flights_data::db_schema::{Itineraries, MonitoredTrips, TripRoutes};

#[derive(Template)]
#[template(path = "trips.html")]
struct TripsTemplate {
    trips: Vec<TripListing>,
}

#[derive(sqlx::FromRow, Debug)]
struct TripListing {
    id: i32,
    name: String,
    min_price: i16,
}

pub async fn trips(pool: web::Data<PgPool>) -> impl Responder {
    let trips = get_trips(&pool).await;
    let template = TripsTemplate { trips };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

async fn get_trips(pool: &web::Data<PgPool>) -> Vec<TripListing> {
    let monitored_trips = Alias::new("m");
    let trip_routes = Alias::new("r");
    let itineraries = Alias::new("i");
    let min_price_column = Alias::new("min_price");

    let query = Query::select()
        .columns(vec![
            (monitored_trips.clone(), MonitoredTrips::Id),
            (monitored_trips.clone(), MonitoredTrips::Name),
        ])
        .expr_as(
            Func::min(Expr::col((itineraries.clone(), Itineraries::Price))),
            min_price_column,
        )
        .from_as(MonitoredTrips::Table, monitored_trips.clone())
        .join_as(
            JoinType::Join,
            TripRoutes::Table,
            trip_routes.clone(),
            Expr::col((monitored_trips.clone(), MonitoredTrips::Id))
                .equals((trip_routes.clone(), TripRoutes::MonitoredTripId)),
        )
        .join_as(
            JoinType::LeftJoin,
            Itineraries::Table,
            itineraries.clone(),
            Expr::col((trip_routes.clone(), TripRoutes::FromAirportId))
                .equals((itineraries.clone(), Itineraries::FromAirportId))
                .and(
                    Expr::col((trip_routes.clone(), TripRoutes::ToAirportId))
                        .equals((itineraries.clone(), Itineraries::ToAirportId)),
                ),
        )
        .and_where(Expr::col((monitored_trips.clone(), MonitoredTrips::UserId)).eq(1))
        .group_by_col((monitored_trips.clone(), MonitoredTrips::Id))
        .to_string(PostgresQueryBuilder);

    return sqlx::query_as(&query)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
}
