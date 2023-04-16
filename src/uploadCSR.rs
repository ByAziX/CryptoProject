use actix_web::{ post, HttpResponse, Responder,Error
};
use tera::Tera;


use actix_multipart::{
    form::{
        tempfile::{TempFile},
        MultipartForm,
    },
    
};

#[derive(Debug, MultipartForm)]
pub(crate) struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/upload/certificate")]
pub(crate) async fn save_files(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    for f in form.files {
        let path = format!("./tmp/{}", f.file_name.unwrap());
        log::info!("saving to {path}");
        f.file.persist(path).unwrap();
    }

    Ok(HttpResponse::Ok())
}