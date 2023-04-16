use std::fs::OpenOptions;
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
pub(crate) struct FormData {
    email: String,
    otp: String,
}


// generate_otp() génère un OTP aléatoire et l'envoie par e-mail à l'utilisateur
pub(crate) async fn generate_otp(email: String) {
    let mut rng = rand::thread_rng();
    let otp: u32 = rng.gen_range(100000..999999);
    let otp = otp.to_string();
    let otp = otp.as_bytes();
    send_otp_email(email.clone(), otp.clone()).await;

    let mut file = File::open("otp.txt").unwrap();
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

        let mut file = File::create("otp.txt").expect("Failed to create file otp.txt");
        file.write_all(updated_contents.as_bytes())
            .expect("Failed to write to file otp.txt");
    } else {
        let file = OpenOptions::new().append(true).create(true).open("otp.txt");
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

async fn send_otp_email(email_user:String,otp: &[u8]) {
    let email = Message::builder()
        .from("projetcryptoca@gmail.com".parse().unwrap())
        .to(email_user.parse().unwrap())
        .subject("OTP")
        .body(String::from("Votre OTP est : ") + String::from_utf8_lossy(otp).as_ref())
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

    mailer.send(&email).unwrap();
    println!("Email envoyé avec succès!");
}

// verify_otp() vérifie si l'OTP entré par l'utilisateur correspond à celui envoyé par e-mail
pub(crate) async fn verify_otp(email: String,otp: &[u8]) -> bool {
    let mut file = File::open("otp.txt").unwrap();
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