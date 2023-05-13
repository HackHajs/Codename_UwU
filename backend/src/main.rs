use std::{
    collections::hash_map::HashMap,
};

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use regex::Regex;

mod players;
use players::{parse_players, save_players, Message, Field};

#[get("/players")]
async fn get_players() -> impl Responder {
    HttpResponse::Ok().insert_header(("Access-Control-Allow-Origin", "*")).json(Message{ players: parse_players() })
}

#[get("/update/{data}")]
async fn update_player(data: web::Path<String>) -> impl Responder {
    let re = Regex::new(r"([^\/]+):([^\/]+):([^\/]+)").unwrap();
    let cap = re.captures_iter(&data).next().unwrap();
    let player = cap[1].to_string();
    let field = &cap[2];
    let value: u32 = cap[3].parse().unwrap();

    let mut players = parse_players();

    match field {
        "health" => {
            players.entry(player).and_modify(|player| player.health += value);
        }
        "xpos" => {
            players.entry(player).and_modify(|players| players.x_pos = value);
        }
        "ypos" => {
            players.entry(player).and_modify(|players| players.y_pos = value);
        }
        _ => (),
    };
    save_players(players);

    HttpResponse::Ok().insert_header(("Access-Control-Allow-Origin", "*")).json("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| App::new().service(get_players).service(update_player))
        .bind("127.0.0.1:8080")?
        .run()
    .await
}
