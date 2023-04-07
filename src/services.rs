use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

pub async fn search_channel(channel: web::Path<String>) -> impl Responder {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&key=", channel))
        .header("User-Agent", "My Rust App")
        .send()
        .await
        .unwrap();
let id = "UCbCmjCuTUZos6Inko4u57UQ";
    let data = response.json::<serde_json::Value>().await.unwrap();

    HttpResponse::Ok().json(data)
}