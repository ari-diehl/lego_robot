use actix_web::{web, App, HttpServer};

use ev3dev_lang_rust::{motors::MotorPort, sensors::SensorPort};
use std::sync::Mutex;

mod manual_control;
mod paper;
mod pen;
mod plotter;
mod routes;

use paper::Paper;
use pen::Pen;
use plotter::Plotter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let plotter = web::Data::new(Mutex::new(Plotter::new(
        Pen::new(MotorPort::OutB, MotorPort::OutA, SensorPort::In2).unwrap(),
        Paper::new(MotorPort::OutC, SensorPort::In1).unwrap(),
    )));

    HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().limit(usize::MAX))
            .app_data(plotter.clone())
            .service(
                web::scope("/api")
                    .service(routes::post_draw)
                    .service(routes::post_controls)
                    .service(routes::get_drawings)
                    .service(routes::post_drawings)
                    .service(routes::get_drawing_instructions),
            )
            .service(actix_files::Files::new("/", "static/").index_file("index.html"))
            .default_service(web::get().to(routes::index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
