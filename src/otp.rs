use std::{fs::File, io::Write};

use actix_web::{web, HttpResponse, Responder, get, post};
use rand::Rng;
use serde::{Deserialize, Serialize};

// Structure pour représenter les données du formulaire
#[derive(Serialize, Deserialize)]
pub(crate) struct FormData {
    email: String,
    otp: String,
}

// Handler pour générer l'OTP
#[get("/generate_otp")]
pub(crate) async fn generate_otp() -> impl Responder {
    let mut rng = rand::thread_rng();
    let otp: u32 = rng.gen_range(100000..999999);
    let otp = otp.to_string();
    let otp = otp.as_bytes();

    
    let file = File::create("./OTPgenerate/otp.txt");
    file.unwrap().write_all(otp).unwrap();
    HttpResponse::Ok().json(serde_json::json!({ "otp": otp }))
}

// pub(crate) async fn checkOTP()

// Handler pour soumettre le formulaire

#[post("/submit_form")]
pub(crate) async fn submit_form(_data: web::Json<FormData>) -> impl Responder {
    
    HttpResponse::Ok().json(serde_json::json!({ "message": "Formulaire soumis avec succès !" }))
}