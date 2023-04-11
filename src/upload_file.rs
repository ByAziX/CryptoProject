use actix_multipart::{
    form::{
        tempfile::{TempFile},
        MultipartForm,
    },
    
};
use actix_web::{Error, HttpResponse, Responder, post};


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


