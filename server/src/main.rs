use actix_web::{web, get, App, HttpResponse, HttpServer};
use reqwest;
use serde::{Deserialize, Serialize};
use reqwest::Response;

#[derive(Deserialize, Serialize, Debug)]
struct RescueLog {
    id: i32,
    date: String,
    productivity_pulse: u8,
    total_hours: f32,
    total_duration_formatted: String,
}

async fn get_rescue_log() -> Result<Response, reqwest::Error> {
    let key = "API_KEY";
    let token = dotenv::var(key).unwrap();

    let url = format!(
        "https://www.rescuetime.com/anapi/daily_summary_feed?key={token}",
        token = token
    );
    let res = reqwest::get(url).await?;
    Ok(res)
}

#[get("/api")]
async fn api() -> Result<HttpResponse, actix_web::Error>{

    let result = get_rescue_log().await?;
    let logs: Vec<RescueLog> = result.json().await?;
    println!("{:?}", logs);
    Ok(HttpResponse::Ok().json(logs))
}

fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| HttpResponse::Ok().body("app")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(|| {
        App::new()
            .configure(app_config)
            .service(api)
            .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
