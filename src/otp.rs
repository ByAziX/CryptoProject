
use std::{fs::File, io::Write};

use actix_web::{web, HttpResponse, Responder, get, post,HttpRequest,App,HttpServer};
use rand::Rng;
use serde::{Deserialize, Serialize};



extern crate lettre;
extern crate lettre_email;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre_email::Email;


// Structure pour représenter les données du formulaire
#[derive(Serialize, Deserialize)]
pub(crate) struct FormData {
    email: String,
    otp: String,
}

// Define the data structure for email payload
#[derive(Debug, Deserialize)]
struct EmailData {
    to: String,
    subject: String,
    body: String,
}

// Handler pour générer l'OTP
#[get("/generate_otp")]
pub(crate) async fn generate_otp() -> impl Responder {
    let mut rng = rand::thread_rng();
    let otp: u32 = rng.gen_range(100000..999999);
    let otp = otp.to_string();
    let otp = otp.as_bytes();

   
send_email(otp).await;
        
    let file = File::create("./OTPgenerate/otp.txt");
    file.unwrap().write_all(otp).unwrap();
    HttpResponse::Ok().json(serde_json::json!({ "otp": otp }))
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

// pub(crate) async fn checkOTP()

// Handler pour soumettre le formulaire

#[post("/submit_form")]
pub(crate) async fn submit_form(_data: web::Json<FormData>) -> impl Responder {
    
    HttpResponse::Ok().json(serde_json::json!({ "message": "Formulaire soumis avec succès !" }))
}