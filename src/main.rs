use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde_json::json;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "rory": {
            "total": 100,
            "id": 1,
            "url": "https://i.rory.cat/OShcAhsR.webp"
        }
    });

    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))}
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
