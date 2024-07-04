use std::vec;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use sea_query::extension::postgres::PgExpr;
use sea_query::{
    Alias, CaseStatement, DynIden, Expr, JoinType, Order, PostgresQueryBuilder, Query, SeaRc,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use flights_data::db_schema::{LocationTypeEnum, Locations};

#[derive(Deserialize)]
struct CityQuery {
    term: String,
}

#[derive(Deserialize)]
struct CountryQuery {
    term: String,
}

#[derive(Deserialize)]
struct SubdivisionQuery {
    country_id: String,
}

#[derive(Deserialize)]
struct LocationsQuery {
    ltype: LocationTypeEnum,
    term: Option<String>,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
struct City {
    id: String,
    name: String,
    country_id: String,
    country_name: String,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
struct Country {
    id: String,
    name: String,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
struct Subdivision {
    id: String,
    name: String,
}

#[get("/api/locations/cities")]
pub async fn fetch_cities(
    _req: HttpRequest,
    query: web::Query<CityQuery>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let table_as: DynIden = SeaRc::new(Alias::new("l"));

    let order_case = CaseStatement::new()
        .case(
            Expr::col((Alias::new("l"), Locations::Name)).ilike(query.term.clone()),
            1,
        )
        .case(
            Expr::col((Alias::new("l"), Locations::Name)).ilike(format!("{}%", query.term)),
            2,
        )
        .finally(3);

    let query = Query::select()
        .from_as(Locations::Table, table_as.clone())
        .join_as(
            JoinType::InnerJoin,
            Locations::Table,
            Alias::new("c"),
            Expr::col((Alias::new("l"), Locations::CountryId))
                .equals((Alias::new("c"), Locations::Id)),
        )
        .columns(vec![
            (Alias::new("l"), Locations::Id),
            (Alias::new("l"), Locations::Name),
            (Alias::new("l"), Locations::CountryId),
        ])
        .expr_as(
            Expr::col((Alias::new("c"), Locations::Name)),
            Alias::new("country_name"),
        )
        .expr_as(order_case, Alias::new("order_column"))
        .and_where(
            Expr::col((Alias::new("l"), Locations::LocationType))
                .eq(LocationTypeEnum::City.to_string()),
        )
        .and_where(Expr::col((Alias::new("l"), Locations::Name)).ilike(format!("%{}%", query.term)))
        .order_by(Alias::new("order_column"), Order::Asc)
        .limit(25)
        .to_string(PostgresQueryBuilder);

    let cities: Vec<City> = sqlx::query_as(&query)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    return HttpResponse::Ok().json(cities);
}

#[get("/api/locations/countries")]
pub async fn fetch_countries(
    _req: HttpRequest,
    query: web::Query<CountryQuery>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let order_case = CaseStatement::new()
        .case(Expr::col(Locations::Name).ilike(query.term.clone()), 1)
        .case(
            Expr::col(Locations::Name).ilike(format!("{}%", query.term)),
            2,
        )
        .finally(3);

    let query = Query::select()
        .columns(vec![Locations::Id, Locations::Name])
        .from(Locations::Table)
        .expr_as(order_case, Alias::new("order_column"))
        .and_where(Expr::col(Locations::LocationType).eq(LocationTypeEnum::Country.to_string()))
        .and_where(Expr::col(Locations::Name).ilike(format!("%{}%", query.term)))
        .order_by(Alias::new("order_column"), Order::Asc)
        .limit(25)
        .to_string(PostgresQueryBuilder);

    let countries: Vec<Country> = sqlx::query_as(&query)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    return HttpResponse::Ok().json(countries);
}

#[get("/api/locations/subdivisions")]
pub async fn fetch_subdivisions(
    _req: HttpRequest,
    query: web::Query<SubdivisionQuery>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let query = Query::select()
        .columns(vec![Locations::Id, Locations::Name])
        .from(Locations::Table)
        .and_where(Expr::col(Locations::LocationType).eq(LocationTypeEnum::Subdivision.to_string()))
        .and_where(Expr::col(Locations::CountryId).eq(query.country_id.clone()))
        .to_string(PostgresQueryBuilder);

    let subdivisions: Vec<Subdivision> = sqlx::query_as(&query)
        .fetch_all(pool.get_ref())
        .await
        .unwrap();

    return HttpResponse::Ok().json(subdivisions);
}
