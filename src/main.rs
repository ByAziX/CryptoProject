use actix_multipart::{
    form::{
        tempfile::{TempFileConfig},
    },
    
};
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use tera::Tera;
mod upload_file;
mod otp;

#[derive(Debug, serde::Deserialize)]
struct FormData {
    email: String,
}

// Traitement de la requête GET pour la route /
async fn index(tera: web::Data<Tera>) -> HttpResponse {
    let context = tera::Context::from_serialize(serde_json::json!({})).expect("Erreur lors de la sérialisation des données");
    let rendered = tera.render("index.html", &context).expect("Erreur lors du rendu du template index");
    HttpResponse::Ok().body(rendered)
}

// Traitement de la requête POST pour la route /otp
pub(crate) async fn email_submit_otp_generation(tera: web::Data<Tera>, form: web::Form<FormData>) -> HttpResponse {

    otp::generate_otp(form.email.to_string()).await;
    
    
    let context = tera::Context::from_serialize(serde_json::json!({ "email": form.email })).expect("Erreur lors de la sérialisation des données");
    let rendered = tera.render("otp.html", &context).expect("Erreur lors du rendu du template otp");
    HttpResponse::Ok().body(rendered)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("creating temporary upload directory");
    std::fs::create_dir_all("./tmp")?;

    log::info!("starting HTTP server at http://localhost:8080");
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*")).unwrap();


    HttpServer::new(move || {
        App::new()
            .data(tera.clone())
            .wrap(middleware::Logger::default())
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/otp").route(web::post().to(email_submit_otp_generation)))
            
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}

