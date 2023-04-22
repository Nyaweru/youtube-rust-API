use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::process::id;
use std::fs;
pub struct search_channel{
    tittle:String,
    id:i32,
    description:String,
}
pub async fn search_channel(channel_id: web::Path<String>) -> impl Responder {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("https://www.googleapis.com/youtube/v3/channels?part=snippet&id={}&key=AIzaSyB3tcaU0BeNvvTPbvwm_5PAxE2uxft3M5w", channel_id))
        .header("User-Agent", "My Rust App")
        .send()
        .await
        .unwrap();

    let data = response.json::<serde_json::Value>().await.unwrap();
    let title = data["items"][0]["snippet"]["title"].as_str().unwrap();
    let description = data["items"][0]["snippet"]["description"].as_str().unwrap();
    let image_url = data["items"][0]["snippet"]["thumbnails"]["default"]["url"].as_str().unwrap();

    let video_response = client
        .get(&format!("https://www.googleapis.com/youtube/v3/search?part=snippet&channelId={}&maxResults=8&order=date&type=video&key=AIzaSyB3tcaU0BeNvvTPbvwm_5PAxE2uxft3M5w", channel_id))
        .header("User-Agent", "My Rust App")
        .send()
        .await
        .unwrap();

    let video_data = video_response.json::<serde_json::Value>().await.unwrap();
    let videos = video_data["items"].as_array().unwrap();

    let mut video_html = String::new();
    for video in videos {
        let video_title = video["snippet"]["title"].as_str().unwrap();
        let video_id = video["id"]["videoId"].as_str().unwrap();
        let thumbnail_url = video["snippet"]["thumbnails"]["default"]["url"].as_str().unwrap();
        video_html.push_str(&format!("<li><a href=\"https://www.youtube.com/watch?v={}\"><img src=\"{}\" alt=\"Video Thumbnail\">{}</a></li>", video_id, thumbnail_url, video_title));
    }

    let html = format!("<html><body><h1>{}</h1><img src=\"{}\" alt=\"Channel Image\"><p>{}</p><h2>Recent Videos:</h2><ul>{}</ul></body></html>", title, image_url, description, video_html);

    
    fs::write("index.html", html).expect("Unable to write file");

    HttpResponse::Ok().body("HTML written to index.html")
}


 



        
    
    




