use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        MultipartForm,
    },
    Multipart,
};
use actix_web::{
    cookie::Cookie, get, middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use futures_util::{StreamExt, TryStreamExt};
use tera::Tera;

mod openssl_cmd;
mod otp;
mod send_certificates;

#[derive(Debug, serde::Deserialize)]
struct FormDataEmail {
    email: String,
}
#[derive(Debug, serde::Deserialize)]

struct FormDataOtp {
    otp: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    file: Option<TempFile>,
}

#[get("/")]
// Traitement de la requête GET pour la route /
async fn index(tera: web::Data<Tera>) -> HttpResponse {
    let context = tera::Context::from_serialize(serde_json::json!({}))
        .expect("Erreur lors de la sérialisation des données");
    let rendered = tera
        .render("index.html", &context)
        .expect("Erreur lors du rendu du template index");
    HttpResponse::Ok().body(rendered)
}

// Traitement de la requête POST pour la route /otp
#[post("/otp")]
async fn email_submit_otp_generation(
    tera: web::Data<Tera>,
    form: web::Form<FormDataEmail>,
) -> HttpResponse {
    otp::generate_otp(form.email.to_string()).await;

    // Stocker l'e-mail dans un cookie pour une utilisation ultérieure
    let mut cookie = Cookie::new("email", form.email.to_string());
    cookie.set_path("/uploadCSR");
    cookie.set_http_only(true);
    cookie.set_secure(true);

    let context = tera::Context::from_serialize(serde_json::json!({ "email": form.email}))
        .expect("Erreur lors de la sérialisation des données");
    let rendered = tera
        .render("otp.html", &context)
        .expect("Erreur lors du rendu du template otp");

    HttpResponse::Ok().cookie(cookie).body(rendered)
}

// Traitement de la requête POST pour la route /uploadCSR
#[post("/uploadCSR")]
async fn verification_otp(
    tera: web::Data<Tera>,
    form: web::Form<FormDataOtp>,
    req: HttpRequest,
) -> HttpResponse {
    // Récupérer la variable depuis le cookie
    let cookie = req.cookie("email");
    if let Some(cookie) = cookie {
        let email = cookie.value();

        let verify_otp = otp::verify_otp(email.to_string(), form.otp.as_bytes()).await;

        if verify_otp {
            let context = tera::Context::from_serialize(serde_json::json!({ "email": email }))
                .expect("Erreur lors de la sérialisation des données");
            let rendered = tera
                .render("upload_csr.html", &context)
                .expect("Erreur lors du rendu du template uploadCSR");

            HttpResponse::Ok().cookie(cookie).body(rendered)
        } else {
            // httpResponse error message
            HttpResponse::Ok().body("404 error OTP incorrect")
        }
    } else {
        HttpResponse::Ok().body("404 error mail not found in")
    }
}
#[post("/MyCertificates")]
async fn create_certificates(
    tera: web::Data<Tera>,
    MultipartForm(form): MultipartForm<UploadForm>,
    req: HttpRequest,
) -> HttpResponse {
    let cookie = req.cookie("email");

    if let Some(cookie) = cookie {
        let email = cookie.value();

        if let Some(file) = form.file {
            let path = format!("./tmp/{}.csr", email);
            file.file.persist(path.clone()).unwrap();

            if openssl_cmd::check_csr(email.to_string(), &path).await {
                openssl_cmd::create_cert(email.to_string(), &path).await;

                let context = tera::Context::from_serialize(serde_json::json!({ "email": email }))
                    .expect("Erreur lors de la sérialisation des données");
                let rendered = tera
                    .render("MyCertificates.html", &context)
                    .expect("Erreur lors du rendu du template uploadCSR");

                HttpResponse::Ok().cookie(cookie).body(rendered)
            } else {
                HttpResponse::Ok().body("404 error csr incorrect")
            }
        } else {
            HttpResponse::Ok().body("404 error file not found in")
        }
    } else {
        HttpResponse::Ok().body("404 error mail not found in")
    }
}

#[post("/MyCertificates/send_certificate_with_email")]
async fn send_all_certificates_to_user(
    tera: web::Data<Tera>,
    req: HttpRequest,
) -> HttpResponse {
    let cookie = req.cookie("email");

    if let Some(cookie) = cookie {
        let email = cookie.value();

        send_certificates::send_cert(email.to_string());

        let context = tera::Context::from_serialize(serde_json::json!({ "email": email }))
        .expect("Erreur lors de la sérialisation des données");
         let rendered = tera
        .render("MyCertificates.html", &context)
        .expect("Erreur lors du rendu du template uploadCSR");

    HttpResponse::Ok().cookie(cookie).body(rendered)
        
    } else {
        HttpResponse::Ok().body("404 error mail not found in")
    }
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
            .service(index)
            .service(email_submit_otp_generation)
            .service(verification_otp)
            .service(create_certificates)
            .service(send_all_certificates_to_user)
            .service(
                actix_files::Files::new("/static", "./src/static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
