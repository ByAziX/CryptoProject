use actix_multipart::{
    form::{
        tempfile::{TempFileConfig},
    },
    
};
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};

mod upload_file;
mod otp;


async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("./templates/index.html"))
}

async fn upload_file() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("./templates/fileCSRUpload.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("creating temporary upload directory");
    std::fs::create_dir_all("./tmp")?;

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(web::resource("/").route(web::get().to(index)))
            .service(otp::generate_otp)
            .service(otp::submit_form)
            .service(web::resource("/upload").route(web::get().to(upload_file)))
            .service(upload_file::save_files)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}

