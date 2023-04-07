use actix_cors::Cors;
use actix_web::{ web, App, HttpServer,http};
mod services;
use handlebars::Handlebars;
use actix_web::get;
use services::{search_channel};
use actix_web::{HttpResponse, Responder};
use serde::Serialize;
#[derive(Serialize)]
struct ChannelInfo {
    api_key:String,
    channel_id:String,
    channel_name: String,
    channel_description: String,
    num_subscribers: u32,
}

#[get("/channel-info")]
async fn channel_info() -> impl Responder {
    let channel_info = ChannelInfo {
        api_key: "AIzaSyB3tcaU0BeNvvTPbvwm_5PAxE2uxft3M5w".to_string(),
        channel_id: "UCbCmjCuTUZos6Inko4u57UQ".to_string(),
        channel_name: "My Channel".to_string(),
        channel_description: "This is my channel".to_string(),
        num_subscribers: 1000,
    };





    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("channel-info", "<h1>{{channel_name}}</h1><p>{{channel_description}}</p><p>{{num_subscribers}} subscribers</p>")
        .unwrap();

    let html = handlebars.render("channel-info", &channel_info).unwrap();

    HttpResponse::Ok().body(html)

    }

    fn parse_youtube_data(response: &str) -> Result<serde_json::Value, serde_json::Error> {
        let data: serde_json::Value = serde_json::from_str(response)?;
        Ok(data)
    }
    


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
         let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8080")
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_methods(vec!["GET", "POST","PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .service(web::resource("/{channel}").to(search_channel))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
