use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use sea_query::extension::postgres::PgExpr;
use sea_query::{
    Alias, CaseStatement, DynIden, Expr, JoinType, Order, PostgresQueryBuilder, Query, SeaRc,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use flights_data::db_schema::{LocationTypeEnum, Locations, Trip};

#[derive(Deserialize)]
struct CityQuery {
    term: String,
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

#[get("/locations/city")]
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
