use std::env;
use std::fs::{OpenOptions};
use std::io::Read;
use std::{fs::File, io::Write};
use rand::Rng;
use serde::{Deserialize, Serialize};

extern crate lettre;
extern crate lettre_email;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};


// Structure pour représenter les données du formulaire
#[derive(Serialize, Deserialize)]
pub struct FormData {
    email: String,
    otp: String,
}


// generate_otp() génère un OTP aléatoire et l'envoie par e-mail à l'utilisateur

pub async fn generate_otp(email: String,path_file : String,subject:String,message:String) {
    let mut rng = rand::thread_rng();
    let otp: u32 = rng.gen_range(100000..999999);
    let otp = otp.to_string();
    let otp = otp.as_bytes();

    send_otp_email(email.clone(), otp,subject,message).await;

    
        

        let mut file = File::open(path_file.clone()+".txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        if contents.contains(&email) {
            // Update OTP in the file
            let mut updated_contents = String::new();
            for line in contents.lines() {
                if line.starts_with(&email) {
                    // Update OTP for the matching email
                    let new_line = format!("{}:{}", email, String::from_utf8_lossy(&otp));
                    updated_contents.push_str(&new_line);
                } else {
                    updated_contents.push_str(line);
                }
                updated_contents.push('\n');
            }

            let mut file = File::create(path_file.clone()+".txt").expect("Failed to create file otp.txt");
            file.write_all(updated_contents.as_bytes())
                .expect("Failed to write to file otp.txt");
        } else {
            let file = OpenOptions::new().append(true).create(true).open(path_file.clone()+".txt");
            let mut file = match file {
                Ok(file) => file,
                Err(_) => panic!("Impossible d'ouvrir le fichier otp.txt"),
            };
            file.write_all(email.as_bytes()).unwrap();
            file.write_all(b":").unwrap();
            file.write_all(otp).unwrap();
            file.write_all(b"\n").unwrap();
        }
    

    println!("Email et OTP ajoutés au fichier otp.txt avec succès!");
}


async fn send_otp_email(email_user:String,otp: &[u8],subject:String,message:String) {
    let email = Message::builder()
        .from("projetcryptoca@gmail.com".parse().unwrap())
        .to(email_user.parse().unwrap())
        .subject(subject)
        .body(String::from(message) + String::from_utf8_lossy(otp).as_ref())
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
        Ok(_) => println!("Email envoyé avec succès!"),
        Err(e) => println!("Erreur lors de l'envoi de l'e-mail: {:?}", e),
    }
}

// verify_otp() vérifie si l'OTP entré par l'utilisateur correspond à celui envoyé par e-mail
pub async fn verify_otp(email: String,otp: &[u8],path_file:String) -> bool {
    let mut file = File::open(path_file+".txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    if contents.contains(&email) {
        for line in contents.lines() {
            if line.starts_with(&email) {
                let otp_file = line.split(":").collect::<Vec<&str>>()[1];
                if otp_file == String::from_utf8_lossy(otp).as_ref() {
                    return true;
                } else {
                    return false;
                }
            }
        }
    } else {
        return false
    }
    return false;

}


