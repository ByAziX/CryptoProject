
extern crate openssl;


extern crate lettre;
extern crate lettre_email;


use std::{fs, env};

use lettre::message::header::ContentType;
use lettre::message::{SinglePart, MultiPart, Attachment};




use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};



pub fn send_cert(email_user: String) {
    let filename = email_user.clone() + ".pem";
    let file_body = fs::read("./new_certs_client/".to_owned()+&filename).unwrap();
    let content_type = ContentType::parse("application/x-pem-file").unwrap();
    let attachment1 = Attachment::new(filename).body(file_body, content_type);

    let filename = "cacert.pem";
    let file_body = fs::read("./Certificats/offline/ACI/".to_owned()+&filename).unwrap();
    let content_type = ContentType::parse("application/x-pem-file").unwrap();
    let attachment2 = Attachment::new("ISEN_ACI.pem".to_owned()).body(file_body, content_type);

    let filename = "cacert.pem";
    let file_body = fs::read("./Certificats/offline/ACR/".to_owned()+&filename).unwrap();
    let content_type = ContentType::parse("application/x-pem-file").unwrap();
    let attachment3 = Attachment::new("ISEN_ACR.pem".to_owned()).body(file_body, content_type);

    let email = Message::builder()
        .from("projetcryptoca@gmail.com".parse().unwrap())
        .to(email_user.parse().unwrap())
        .subject("File")
        .multipart(
            MultiPart::mixed()
                .singlepart(SinglePart::builder().body(String::from("Votre certificats est joint ci-dessus : ")))
                .multipart(
                    MultiPart::mixed()
                        .singlepart(attachment1)
                        .singlepart(attachment2)
                        .singlepart(attachment3)

                )
                
        )
        .unwrap();

        let secret = env::var("EMAIL_SECRET");


        let creds = Credentials::new(
            "projetcryptoca@gmail.com".to_string(),
            secret.unwrap(),
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


