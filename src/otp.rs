
use std::fs::OpenOptions;
use std::io::Read;
use std::{fs::File, io::Write};

use actix_web::{web, HttpResponse, Responder, get, post,HttpRequest,App,HttpServer};
use rand::Rng;
use serde::{Deserialize, Serialize};



extern crate lettre;
extern crate lettre_email;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre_email::Email;

use tera::Tera;



// Structure pour représenter les données du formulaire
#[derive(Serialize, Deserialize)]
pub(crate) struct FormData {
    email: String,
    otp: String,
}


pub(crate) async fn generate_otp(Email: String)  {
    let mut rng = rand::thread_rng();
    let otp: u32 = rng.gen_range(100000..999999);
    let otp = otp.to_string();
    let otp = otp.as_bytes();



    // check if email is already in file
    let mut file = File::open("otp.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    if contents.contains(&Email) {
        // changer otp dans le fichier apres les :
        
        return;
    }else {
        let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("otp.txt");
        let mut file = match file {
            Ok(file) => file,
            Err(_) => panic!("Impossible d'ouvrir le fichier otp.txt"),
        };
        file.write_all(Email.as_bytes()).unwrap();
        file.write_all(b":").unwrap();
        file.write_all(otp).unwrap();
        file.write_all(b"\n").unwrap();
    }
    
     
    println!("Email et OTP ajoutés au fichier otp.txt avec succès!");
    
}

async fn send_email(otp:&[u8]) -> HttpResponse {
    let email = Message::builder() 
    .from("projetcryptoca@gmail.com".parse().unwrap()) 
    .to("hugo.millet@isen.yncrea.fr".parse().unwrap()) 
    .subject("OTP") 
    .body(String::from("Votre OTP est : ")+&String::from_utf8_lossy(otp).to_string()) 
    .unwrap(); 
    
    let creds = Credentials::new("projetcryptoca@gmail.com".to_string(), "dqvjnxkzwdjdoktc".to_string()); 
    
    // Open a remote connection to gmail 
    let mailer = SmtpTransport::relay("smtp.gmail.com") 
    .unwrap() 
    .credentials(creds) 
    .build(); 
    

    // Send the email using the SMTP transport
    match mailer.send(&email)  {
        Ok(_) => HttpResponse::Ok().body("Email sent successfully!"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to send email"),
    }
}



