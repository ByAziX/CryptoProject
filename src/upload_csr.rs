pub mod openssl_cmd;

use actix_web::{post, Error, HttpRequest, HttpResponse, Responder};

use actix_multipart::form::{tempfile::TempFile, MultipartForm};

#[derive(Debug, MultipartForm)]
pub(crate) struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/upload/certificate")]
pub(crate) async fn save_files(
    MultipartForm(form): MultipartForm<UploadForm>,
    req: HttpRequest,
) -> Result<impl Responder, Error> {
    let cookie = req.cookie("email");
    if let Some(cookie) = cookie {
        let email = cookie.value();

        for f in form.files {
            
            let path = format!("./tmp/{}", email.to_string() + ".csr");
            log::info!("saving to {path}");
            f.file.persist(path.clone()).unwrap();

            if openssl_cmd::check_csr(email.to_string(), &path).await == true {
                log::info!("csr is valid");
                openssl_cmd::create_cert(email.to_string(),&path).await;
            } else {
                log::info!("csr is not valid");
            }
            
        }

        
    } else {
        return Ok(HttpResponse::Unauthorized());
    }

    Ok(HttpResponse::Ok())
}


