use actix_web::{web, App, HttpResponse, HttpServer};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct RescueLog {
    id: i32,
    date: String,
    productivity_pulse: u8,
    total_hours: f32,
    total_duration_formatted: String,
}

async fn get() -> Result<(Vec<RescueLog>), reqwest::Error> {
    let key = "API_KEY";
    let token = dotenv::var(key).unwrap();

    let url = format!(
        "https://www.rescuetime.com/anapi/daily_summary_feed?key={token}",
        token = token
    );
    let res = reqwest::get(url).await.unwrap();

    let logs: Vec<RescueLog> = res.json().await.unwrap();

    Ok(logs)
}

async fn api_config(cfg: &mut web::ServiceConfig) {

    let result = get().await.unwrap();
    println!(result);
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().content_type("application/json").body("ok")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| HttpResponse::Ok().body("app")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(app_config)
            .service(web::scope("api").configure(api_config))
            .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
