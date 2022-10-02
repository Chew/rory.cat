use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::web::Json;
use diesel::{MysqlConnection, r2d2};
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use handlebars::Handlebars;
use rand::{Rng, thread_rng};
use serde_json::json;
use crate::models::image::Image;
use crate::state::AppState;

// Import needed macros
#[macro_use]
extern crate diesel;
extern crate rand;

mod models;
mod state;
mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// HTML Result
async fn index(state: web::Data<AppState>, hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    // id is a random number 1-10
    let mut rng = thread_rng();

    let conn = state.db_pool.get().unwrap();
    let max_rories = Image::count(&conn).unwrap();
    // Pick a random number between 1 and the max number of rories
    let rory_id = rng.gen_range(1..(max_rories+1));
    let rory = Image::find_by_id(rory_id, &conn).unwrap();

    match rory {
        Some(rory) => {
            let data = json!({
                "rory": {
                    "id": rory.id,
                    "total": max_rories,
                    "url": rory.url,
                },
                "by_id": false,
                "rory_exists": true
            });
            let body = hb.render("index", &data).unwrap();
            HttpResponse::Ok().body(body)
        },
        None => HttpResponse::NotFound().body("Not found"),
    }
}

async fn by_id(state: web::Data<AppState>, id: web::Path<i32>, hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let conn = state.db_pool.get().unwrap();
    let max_rories = Image::count(&conn).unwrap();
    // Pick a random number between 1 and the max number of rories
    let rory_id = id.into_inner();
    let rory = Image::find_by_id(rory_id, &conn).unwrap();

    match rory {
        Some(rory) => {
            let data = json!({
                "rory": {
                    "id": rory.id,
                    "total": max_rories,
                    "url": rory.url,
                },
                "by_id": true,
                "rory_exists": true
            });
            let body = hb.render("index", &data).unwrap();
            HttpResponse::Ok().body(body)
        },
        None => {
            let data = json!({
                "rory_exists": false
            });
            let body = hb.render("index", &data).unwrap();
            HttpResponse::NotFound().body(body)
        }
    }
}

// HTML Result
async fn purr_random(state: web::Data<AppState>) -> HttpResponse {
    // id is a random number 1-10
    let mut rng = thread_rng();

    let conn = state.db_pool.get().unwrap();
    let max_rories = Image::count(&conn).unwrap();
    // Pick a random number between 1 and the max number of rories
    let rory_id = rng.gen_range(1..(max_rories+1));
    let rory = Image::find_by_id(rory_id, &conn).unwrap();

    match rory {
        Some(rory) => HttpResponse::Ok().json(Json(rory)),
        None => HttpResponse::NotFound().body("Not found"),
    }
}

async fn purr_by_id(state: web::Data<AppState>, id: web::Path<i32>) -> HttpResponse {
    let conn = state.db_pool.get().unwrap();
    // Pick a random number between 1 and the max number of rories
    let rory_id = id.into_inner();
    let rory = Image::find_by_id(rory_id, &conn).unwrap();

    match rory {
        Some(rory) => HttpResponse::Ok().json(Json(rory)),
        None => HttpResponse::NotFound().json(json!({"error": "this rory does not exist"})),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_manager = ConnectionManager::<MysqlConnection>::new(
        std::env::var("DATABASE_URL").expect("Missing ENV")
    );
    let db_pool: DbPool = r2d2::Pool::builder()
        .build(db_manager)
        .expect("Failed to create DB pool.");

    let app_state = web::Data::new(AppState {
        db_pool: db_pool.clone()
    });

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .app_data(app_state.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
            .route("/id/{id}", web::get().to(by_id))
            // api
            .route("/purr", web::get().to(purr_random))
            .route("/purr/{id}", web::get().to(purr_by_id))
    }).bind(("127.0.0.1", 8080))?.run().await
}
