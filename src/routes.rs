use actix_files::NamedFile;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use glob::glob;
use serde::Deserialize;
use serde_json;
use std::{fs, sync::Mutex};

use crate::manual_control::ManualControl;
use crate::paper::PaperControls;
use crate::pen::PenControls;
use crate::plotter::Plotter;

pub async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[derive(Deserialize)]
pub struct PostDraw {
    drawing_instructions: Vec<Vec<(i32, i32)>>,
}

#[post("/draw")]
pub async fn post_draw(
    draw_request: web::Json<PostDraw>,
    plotter: web::Data<Mutex<Plotter>>,
) -> impl Responder {
    plotter
        .lock()
        .unwrap()
        .draw(&draw_request.drawing_instructions)
        .unwrap();

    HttpResponse::Ok()
}

#[derive(Deserialize)]
pub struct PostControls {
    component: String,
    command: String,
}

#[post("/controls")]
pub async fn post_controls(
    controls_request: web::Json<PostControls>,
    plotter: web::Data<Mutex<Plotter>>,
) -> impl Responder {
    let plotter = plotter.lock().unwrap();

    if controls_request.component == "pen" {
        plotter
            .get_pen()
            .controls(match controls_request.command.as_str() {
                "left" => PenControls::Left,
                "right" => PenControls::Right,
                "up" => PenControls::Up,
                "down" => PenControls::Down,
                "stop" => PenControls::Stop,
                _ => {
                    return HttpResponse::BadRequest();
                }
            })
            .unwrap();
    } else {
        plotter
            .get_paper()
            .controls(match controls_request.command.as_str() {
                "in" => PaperControls::In,
                "out" => PaperControls::Out,
                "stop" => PaperControls::Stop,
                _ => {
                    return HttpResponse::BadRequest();
                }
            })
            .unwrap();
    }

    HttpResponse::Ok()
}

#[get("/drawings")]
pub async fn get_drawings() -> impl Responder {
    let drawings: Vec<String> = glob("static/drawings/*.png")
        .unwrap()
        .map(|entry| {
            entry
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned()
        })
        .collect();

    HttpResponse::Ok().json(drawings)
}

#[derive(Deserialize)]
pub struct PostDrawings {
    name: String,
    drawing: String,
    drawing_instructions: String,
}

#[post("/drawings")]
pub async fn post_drawings(drawing_request: web::Json<PostDrawings>) -> impl Responder {
    image::load_from_memory_with_format(
        &base64::decode(&drawing_request.drawing).unwrap(),
        image::ImageFormat::Png,
    )
    .unwrap()
    .save(format!(
        "{}{}.png",
        "static/drawings/", drawing_request.name
    ))
    .unwrap();
    fs::write(
        format!("drawing_instructions/{}.json", drawing_request.name),
        &drawing_request.drawing_instructions,
    )
    .unwrap();

    HttpResponse::Ok()
}

#[derive(Deserialize)]
pub struct DrawingInstructionsQuery {
    name: String,
}

#[get("/drawing_instructions")]
pub async fn get_drawing_instructions(
    query: web::Query<DrawingInstructionsQuery>,
) -> impl Responder {
    HttpResponse::Ok().json(
        serde_json::from_str::<Vec<Vec<(i32, i32)>>>(
            fs::read_to_string(format!("drawing_instructions/{}.json", query.name))
                .unwrap()
                .as_str(),
        )
        .unwrap(),
    )
}
