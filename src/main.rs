
use actix_http::body::BoxBody;
use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        MultipartForm,
    },
};
use actix_web::{
    cookie::Cookie, get,http::{header::ContentType, StatusCode},    error,  middleware::{self, ErrorHandlerResponse, ErrorHandlers},Result, post, web, App, HttpRequest, HttpResponse, HttpServer, dev::ServiceResponse,Error, Responder,
};
use actix_web_lab::respond::Html;
use tera::Tera;

use std::collections::HashMap;



mod openssl_cmd;
mod otp;
mod certificates;

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
async fn index(tmpl: web::Data<tera::Tera>) -> Result<impl Responder, Error> {
   

    let context = tera::Context::from_serialize(serde_json::json!({}))
        .expect("Erreur lors de la sérialisation des données");
    let rendered = tmpl
        .render("index.html", &context)
        .expect("Erreur lors du rendu du template index");
    Ok(HttpResponse::Ok().body(rendered))

}

// Traitement de la requête POST pour la route /otp
#[post("/otp")]
async fn email_submit_otp_generation(
    tera: web::Data<Tera>,
    form: web::Form<FormDataEmail>,
) -> HttpResponse {
    // generate otp 2 time without crashing page
    otp::generate_otp(form.email.to_string(),"otp".to_string()).await;

    



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
                if openssl_cmd::create_cert(email.to_string(), &path).await {
                    //otp::generate_otp(email.to_string(),"otp_revoke".to_string()).await;                    
                    let context = tera::Context::from_serialize(serde_json::json!({ "email": email }))
                    .expect("Erreur lors de la sérialisation des données");
                let rendered = tera
                    .render("MyCertificates.html", &context)
                    .expect("Erreur lors du rendu du template uploadCSR");
                
                HttpResponse::Ok().cookie(cookie).body(rendered)
                } else {
                    let context = tera::Context::from_serialize(serde_json::json!({ "email": email }))
                    .expect("Erreur lors de la sérialisation des données");
                let rendered = tera
                    .render("upload_csr.html", &context)
                    .expect("Erreur lors du rendu du template uploadCSR");

                HttpResponse::Ok().cookie(cookie).body(rendered)

                }

                
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

        certificates::send_cert(email.to_string());

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

#[post("/revoke_certificate")]
async fn revoke_certificate(
    tera: web::Data<Tera>,
    req: HttpRequest,
) -> HttpResponse {
    let cookie = req.cookie("email");

    if let Some(cookie) = cookie {
        let email = cookie.value();

        openssl_cmd::revoke_cert(email.to_string());

        let context = tera::Context::from_serialize(serde_json::json!({ "email": email }))
        .expect("Erreur lors de la sérialisation des données");
         let rendered = tera
        .render("upload_csr.html", &context)
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
            .app_data(web::Data::new(tera.clone()))
            .wrap(middleware::Logger::default())
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(index)
            .service(email_submit_otp_generation)
            .service(verification_otp)
            .service(create_certificates)
            .service(send_all_certificates_to_user)
            .service(revoke_certificate)
            .service(
                actix_files::Files::new("/static", "./src/static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(web::scope("").wrap(error_handlers()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(e.to_string())
    };

    let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());
    match tera {
        Some(tera) => {
            let mut context = tera::Context::new();
            context.insert("error", error);
            context.insert("status_code", res.status().as_str());
            let body = tera.render("error.html", &context);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}