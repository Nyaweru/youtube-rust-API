use actix_cors::Cors;
use actix_web::{ web, App, HttpServer,http};
mod services;
use handlebars::Handlebars;
use actix_web::get;
use services::{search_channel};
use actix_web::{HttpResponse, Responder};
use serde::Serialize;
use reqwest::get;
//use crate::services::index;
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
       .wrap(cors)
            
           .service(web::resource("/{channel_id}").to(search_channel))
            //.service(index)
   })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
