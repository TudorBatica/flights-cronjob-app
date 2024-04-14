use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(Debug, serde::Deserialize)]
pub struct MonitoredRoute {
    from_airport_code: String,
    to_country_code: String,
    budget: i16,
    trip_type: i16, // todo: change to something safer
}

pub async fn monitor_routes(
    routes: web::Json<Vec<MonitoredRoute>>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    for route in routes.0 {
        match sqlx::query!(
            r#"
        INSERT INTO monitored_routes (airport_code, country_code, budget, trip_type, monitored_by)
        VALUES ($1, $2,  $3, $4, $5)
        "#,
            route.from_airport_code,
            route.to_country_code,
            route.budget,
            route.trip_type,
            1
        )
        .execute(pool.get_ref())
        .await
        {
            Ok(_) => {}
            Err(e) => {
                println!("Could not execute insert query: {}", e);
                return HttpResponse::InternalServerError().finish();
            }
        }
    }

    HttpResponse::Ok().finish()
}
