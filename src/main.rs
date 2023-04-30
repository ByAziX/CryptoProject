mod certificates;
mod openssl_cmd;
mod otp;

use std::env;

use dotenv::dotenv;

use actix_http::body::BoxBody;
use actix_multipart::form::{
    tempfile::{TempFile, TempFileConfig},
    MultipartForm,
};
use actix_web::{
    cookie::Cookie,
    dev::ServiceResponse,
    get,
    http::{header::ContentType, StatusCode},
    middleware::{self, ErrorHandlerResponse, ErrorHandlers},
    post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use serde::{Serialize, Deserialize};
use tera::Tera;

use jsonwebtoken::{encode, Header, EncodingKey, DecodingKey, Validation, decode};

use jsonwebtoken::{Algorithm};
use std::time::{Duration, SystemTime};
#[derive(Debug, serde::Deserialize)]
struct FormDataEmail {
    email: String,
}


#[derive(Debug, serde::Deserialize)]

struct FormDataOtp {
    otp: String,
}
#[derive(Debug, serde::Deserialize)]
struct FormDataRevoke {
    otp: String,
    message: String,
}
// Définissez une struct pour représenter les claims JWT
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    email: String,
    iat: u64,
    exp: u64,
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
async fn email_submit_otp_generation(form: web::Form<FormDataEmail>) -> HttpResponse {
    // generate otp 2 time without crashing page
    
    otp::generate_otp(
        form.email.to_string(),
        "otp".to_string(),
        "OTP".to_string(),
        "Voici votre OTP :".to_string(),
    )
    .await;

    let jwt = generate_jwt(form.email.to_string());

    
    // Stocker l'e-mail dans un cookie pour une utilisation ultérieure
    let mut cookie = Cookie::new("jwt",jwt);
    cookie.set_path("/uploadCSR");
    cookie.set_http_only(true);
    cookie.set_secure(true);

    get_page_response(
        form.email.to_string(),
        "".to_string(),
        cookie,
        "otp.html".to_string(),
    )
}

// Traitement de la requête POST pour la route /uploadCSR
#[post("/uploadCSR")]
async fn verification_otp(form: web::Form<FormDataOtp>, req: HttpRequest) -> HttpResponse {
    // Récupérer la variable depuis le cookie
    let cookie = req.cookie("jwt");

    if let Some(cookie) = cookie {
        let jwt = cookie.value();

        let email = get_email_from_jwt(jwt).unwrap_or_default();

        if otp::verify_otp(email.to_string(), form.otp.as_bytes(), "otp".to_string()).await {
            get_page_response(
                email.to_string(),
                "".to_string(),
                cookie,
                "upload_csr.html".to_string(),
            )
        } else {
            get_page_response(
                email.to_string(),
                "WRONG OTP".to_string(),
                cookie,
                "otp.html".to_string(),
            )
        }
    } else {
        HttpResponse::Ok().body("404 error mail not found in")
    }
}
#[post("/MyCertificates")]
async fn create_certificates(
    MultipartForm(form): MultipartForm<UploadForm>,
    req: HttpRequest,
) -> HttpResponse {
    // Read the email cookie from the request
    let cookie = req.cookie("jwt");

    // Check if the cookie exists
    if let Some(cookie) = cookie {
        let jwt = cookie.value();
        let email = get_email_from_jwt(jwt).unwrap_or_default();

        // Check if the form contains a file
        if let Some(file) = form.file {
            // Save the file to disk
            let path = format!("./tmp/{}.csr", email);
            file.file.persist(path.clone()).unwrap();

            // Check if the CSR matches the email
            if openssl_cmd::check_csr(email.to_string(), &path).await {
                // Create a certificate
                if openssl_cmd::create_cert(email.to_string(), &path).await {
                    otp::generate_otp(
                        email.to_string(),
                        "otp_revoke".to_string(),
                        "OTP de révocation".to_string(),
                        "Voici votre OTP de révocation pour le certificat: ".to_string(),
                    )
                    .await;
                    certificates::send_cert(email.to_string());

                    // Return a success response
                    get_page_response(
                        email.to_string(),
                        "".to_string(),
                        cookie,
                        "MyCertificates.html".to_string(),
                    )
                } else {
                    // Return an error response
                    get_page_response(
                        email.to_string(),
                        "Veuillez révoquer votre CSR avant d'en créer un autre !".to_string(),
                        cookie,
                        "upload_csr.html".to_string(),
                    )
                }
            } else {
                // Return an error response
                get_page_response(
                    email.to_string(),
                    "Le CSR et votre e-mail ne correspondent pas.".to_string(),
                    cookie,
                    "upload_csr.html".to_string(),
                )
            }
        } else {
            // Return an error response
            get_page_response(
                email.to_string(),
                "Le input pour la transmition de la CSR est vide !".to_string(),
                cookie,
                "upload_csr.html".to_string(),
            )
        }
    } else {
        // Return a 404 error response
        HttpResponse::Ok().body("404 error email not found in")
    }
}

#[post("/MyCertificates/send_certificate_with_email")]
async fn send_all_certificates_to_user(req: HttpRequest) -> HttpResponse {
    let cookie = req.cookie("jwt");

    if let Some(cookie) = cookie {
        let jwt = cookie.value();
        let email = get_email_from_jwt(jwt).unwrap_or_default();


        certificates::send_cert(email.to_string());

        get_page_response(
            email.to_string(),
            "Vos fichiers ont été envoyés !".to_string(),
            cookie,
            "MyCertificates.html".to_string(),
        )
    } else {
        HttpResponse::Ok().body("404 error mail not found in")
    }
}

#[post("/revoke_certificate")]
async fn revoke_certificate(req: HttpRequest,form: web::Form<FormDataRevoke> ) -> HttpResponse {
    let cookie = req.cookie("jwt");

    if let Some(cookie) = cookie {
        let jwt = cookie.value();
        let email = get_email_from_jwt(jwt).unwrap_or_default();


        

        if otp::verify_otp(email.to_string(), form.otp.as_bytes(), "otp_revoke".to_string()).await {
            openssl_cmd::revoke_cert(email.to_string());
            get_page_response(
                email.to_string(),
                "".to_string(),
                cookie.clone(),
                "upload_csr.html".to_string(),
            )
        } else {
            get_page_response(
                email.to_string(),
                "WRONG OTP".to_string(),
                cookie.clone(),
                "upload_csr.html".to_string(),
            )
        }

       
    } else {
        HttpResponse::Ok().body("404 error mail not found in")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("creating temporary upload directory");
    std::fs::create_dir_all("./tmp")?;
    dotenv().ok();

    log::info!("starting HTTP server at http://localhost:8080");
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*")).unwrap();
    let secret = env::var("JWT_SECRET")
    .expect("La clé secrète JWT doit être définie dans la variable d'environnement JWT_SECRET");

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
            .service(web::scope("").wrap(error_handlers()))
            .service(
                actix_files::Files::new("/static", "./src/static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
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

fn get_page_response(
    email: String,
    error_msg: String,
    cookie: Cookie,
    page: String,
) -> HttpResponse {
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*")).unwrap();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        HttpResponse::build(StatusCode::NOT_FOUND)
            .content_type(ContentType::plaintext())
            .body(e.to_string())
    };

    let mut context = tera::Context::new();
    context.insert("email", &email);
    context.insert("error_msg", &error_msg);
    let body = tera.render(&page, &context);

    match body {
        Ok(body) => HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::html())
            .cookie(cookie)
            .body(body),
        Err(_) => fallback(&error_msg),
    }
}


fn generate_jwt(email: String) -> String {
    // Créez un header JWT
    let header = Header::new(Algorithm::HS256);

    // Créez une payload JWT
    let claims = Claims {
        email: email.to_string(),
        iat: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        exp: (SystemTime::now() + Duration::from_secs(3600)).duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
    };

    // Générez la clé secrète JWT à partir de la variable d'environnement JWT_SECRET
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET doit être défini !");
    let key = EncodingKey::from_secret(secret.as_ref());

    // Générez le JWT
    jsonwebtoken::encode(&header, &claims, &key).unwrap()
}



fn get_email_from_jwt(jwt: &str) -> Option<String> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = DecodingKey::from_secret(secret.as_ref());

    let validation = Validation::new(Algorithm::HS256);

    let decoded = decode::<Claims>(jwt, &key, &validation);

    match decoded {
        Ok(token) => Some(token.claims.email),
        Err(_) => None,
    }
}