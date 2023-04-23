
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




pub fn send_cert(email_user: String) {
    let filename = email_user.clone() + ".pem";
    let file_body = fs::read("/home/hugo/ISEN/Cours/Cryptographie/CryptoWebsiteCA/CryptoProject/new_certs_client/".to_owned()+&filename).unwrap();
    let content_type = ContentType::parse("application/x-pem-file").unwrap();
    let attachment = Attachment::new(filename).body(file_body, content_type);


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
