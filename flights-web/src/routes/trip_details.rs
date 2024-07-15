use std::ops::Deref;

use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use sea_query::{Alias, Expr, Func, JoinType, Order, PostgresQueryBuilder, Query};
use serde::Deserialize;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::PgPool;

use flights_data::db_schema::{Itineraries, TripRoutes};

#[derive(Template)]
#[template(path = "trip_details.html")]
struct TripDetailsTemplate {
    trip_id: i32,
    itineraries: Vec<ItineraryListing>,
}

#[derive(sqlx::FromRow)]
struct ItineraryListing {
    from_airport_id: String,
    to_airport_id: String,
    departure_depart_at_utc: DateTime<Utc>,
    return_depart_at_utc: DateTime<Utc>,
    price: i16,
    stopovers: i16,
    over_wk: bool,
}

#[derive(Deserialize)]
pub struct TripPath {
    trip_id: i32,
}

#[derive(Deserialize)]
pub struct TripQuery {
    min_nights: Option<usize>,
    max_nights: Option<usize>,
    stopovers: Option<bool>,
}

pub async fn get_trip_page(
    pool: web::Data<PgPool>,
    path: web::Path<TripPath>,
    query: web::Query<TripQuery>,
) -> impl Responder {
    let trip_id = path.trip_id;
    let itineraries = get_itineraries(pool, trip_id, query.deref()).await;

    let template = TripDetailsTemplate {
        trip_id,
        itineraries,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

async fn get_itineraries(
    pool: web::Data<PgPool>,
    trip_id: i32,
    query: &TripQuery,
) -> Vec<ItineraryListing> {
    let trip_routes = Alias::new("r");
    let itineraries = Alias::new("i");

    let mut statement = Query::select()
        .columns(vec![
            (itineraries.clone(), Itineraries::FromAirportId),
            (itineraries.clone(), Itineraries::ToAirportId),
            (itineraries.clone(), Itineraries::DepartureDepartAtUtc),
            (itineraries.clone(), Itineraries::ReturnDepartAtUtc),
            (itineraries.clone(), Itineraries::Price),
            (itineraries.clone(), Itineraries::Stopovers),
        ])
        .expr_as(
            Func::cust(flights_data::db_schema::OverWeekendFunction).args([
                Expr::col((itineraries.clone(), Itineraries::DepartureArriveAtUtc)).into(),
                Expr::col((itineraries.clone(), Itineraries::ReturnDepartAtUtc)).into(),
            ]),
            Alias::new("over_wk"),
        )
        .from_as(TripRoutes::Table, trip_routes.clone())
        .join_as(
            JoinType::InnerJoin,
            Itineraries::Table,
            itineraries.clone(),
            Expr::col((trip_routes.clone(), TripRoutes::FromAirportId))
                .equals((itineraries.clone(), Itineraries::FromAirportId))
                .and(
                    Expr::col((trip_routes.clone(), TripRoutes::ToAirportId))
                        .equals((itineraries.clone(), Itineraries::ToAirportId)),
                ),
        )
        .and_where(Expr::col((trip_routes.clone(), TripRoutes::MonitoredTripId)).eq(trip_id))
        .order_by((itineraries.clone(), Itineraries::Price), Order::Asc)
        .order_by(
            (itineraries.clone(), Itineraries::DepartureDepartAtUtc),
            Order::Asc,
        )
        .to_owned();

    if let Some(min_nights) = query.min_nights {
        statement.and_where(
            Expr::col((itineraries.clone(), Itineraries::ReturnDepartAtUtc)).gt(Expr::col((
                itineraries.clone(),
                Itineraries::DepartureArriveAtUtc,
            ))
            .add(Expr::cust(format!("interval '{} days'", min_nights)))),
        );
    }

    if let Some(max_nights) = query.max_nights {
        statement.and_where(
            Expr::col((itineraries.clone(), Itineraries::ReturnDepartAtUtc)).lt(Expr::col((
                itineraries.clone(),
                Itineraries::DepartureArriveAtUtc,
            ))
            .add(Expr::cust(format!("interval '{} days'", max_nights)))),
        );
    }

    if let Some(stopovers) = query.stopovers {
        if !stopovers {
            statement.and_where(Expr::col((itineraries.clone(), Itineraries::Stopovers)).eq(0));
        }
    }

    let statement = statement.to_string(PostgresQueryBuilder);
    println!("{}", statement);

    return sqlx::query_as(&statement)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
}
