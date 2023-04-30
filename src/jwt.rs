use jsonwebtoken::{encode, Header, EncodingKey, DecodingKey, Validation, decode};

use jsonwebtoken::{Algorithm};
use serde::{Serialize, Deserialize};
use std::time::{Duration, SystemTime};

// Définissez une struct pour représenter les claims JWT
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    email: String,
    iat: u64,
    exp: u64,
}

pub fn generate_jwt(email: String) -> String {
    // Créez un header JWT
    let header = Header::new(Algorithm::HS256);

    // Créez une payload JWT
    let claims = Claims {
        email: email.to_string(),
        iat: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        exp: (SystemTime::now() + Duration::from_secs(3600)).duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
    };

    // Générez la clé secrète JWT à partir de la variable d'environnement JWT_SECRET
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET doit être défini !");
    let key = EncodingKey::from_secret(secret.as_ref());

    // Générez le JWT
    jsonwebtoken::encode(&header, &claims, &key).unwrap()
}



pub fn get_email_from_jwt(jwt: &str) -> Option<String> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = DecodingKey::from_secret(secret.as_ref());

    let validation = Validation::new(Algorithm::HS256);

    let decoded = decode::<Claims>(jwt, &key, &validation);

    match decoded {
        Ok(token) => Some(token.claims.email),
        Err(_) => None,
    }
}