pub mod openssl_cmd;

extern crate openssl;


extern crate lettre;
extern crate lettre_email;


use std::fs;

use lettre::message::header::ContentType;
use lettre::message::{SinglePart, MultiPart, Attachment};




use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

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
                send_cert(email.to_string());
            } else {
                log::info!("csr is not valid");
            }
            
        }

        
    } else {
        return Ok(HttpResponse::Unauthorized());
    }

    Ok(HttpResponse::Ok())
}



fn send_cert(email_user: String) {
    let filename = String::from("/home/hugo/ISEN/Cours/Cryptographie/CryptoWebsiteCA/CryptoProject/new_certs_client/").to_owned() + &email_user + ".pem";
    let file_body = fs::read(filename.clone()).unwrap();
    let content_type = ContentType::parse("application/x-pem-file").unwrap();
    let attachment = Attachment::new(filename).body(file_body, content_type);

    println!("{:?}", attachment);

    let email = Message::builder()
        .from("projetcryptoca@gmail.com".parse().unwrap())
        .to(email_user.parse().unwrap())
        .subject("File")
        .multipart(
            MultiPart::mixed()
                .singlepart(SinglePart::builder().body(String::from("Votre certificats est joint ci-dessus : ")))
                .singlepart(attachment),
        )
        .unwrap();

        let creds = Credentials::new(
            "projetcryptoca@gmail.com".to_string(),
            "dqvjnxkzwdjdoktc".to_string(),
        );
    
        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();
    
            match mailer.send(&email) {
                Ok(_) => log::info!("Email sent successfully!"),
                Err(e) => panic!("Could not send email: {:?}", e),
            }
}
