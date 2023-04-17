extern crate openssl;
use openssl::x509::X509Req;
use openssl::asn1::Asn1String;
use std::fs::File;
use std::io::{Read, Write};


pub(crate) async fn get_csr_subject(csr_file_path: &str) -> Result<String, String> {
    // Lire le fichier CSR
    let mut file = File::open(csr_file_path).map_err(|e| format!("Erreur lors de l'ouverture du fichier CSR : {}", e))?;
    let mut csr_bytes = Vec::new();
    file.read_to_end(&mut csr_bytes).map_err(|e| format!("Erreur lors de la lecture du fichier CSR : {}", e))?;

    // Charger le CSR en tant qu'objet X509Req
    let csr = X509Req::from_pem(&csr_bytes).map_err(|e| format!("Erreur lors de la lecture du CSR : {}", e))?;

    // Obtenir le sujet du CSR
    let subject = csr.subject_name();
    let subject = subject.entries_by_nid(openssl::nid::Nid::COMMONNAME).next().unwrap().data().as_utf8().unwrap();
    Ok(subject.to_string())
}

pub (crate) async fn check_csr(email:String, csr_file_path: &str) -> bool {
    let subject = get_csr_subject(csr_file_path).await.unwrap();
    if subject == email {
        true
    } else {
        false
    }
}


pub(crate) async fn create_cert(email:String,csr_file_path: &str) {
/*faut implémenter le bon code avec la ACI !!!!!!!!!!!! */
    let mut file = File::open(csr_file_path).unwrap();
    let mut csr_bytes = Vec::new();
    file.read_to_end(&mut csr_bytes).unwrap();

    let csr = X509Req::from_pem(&csr_bytes).unwrap();

    let mut builder = openssl::x509::X509::builder().unwrap();
    builder.set_version(2).unwrap();
    builder.set_subject_name(csr.subject_name()).unwrap();
    builder.set_pubkey(&csr.public_key().unwrap()).unwrap();
    builder.set_not_before(&openssl::asn1::Asn1Time::days_from_now(0).unwrap()).unwrap();
    builder.set_not_after(&openssl::asn1::Asn1Time::days_from_now(365).unwrap()).unwrap();
    builder.sign(&openssl::pkey::PKey::from_rsa(openssl::rsa::Rsa::generate(2048).unwrap()).unwrap(), openssl::hash::MessageDigest::sha256()).unwrap();

    let cert = builder.build();

    let cert_pem = cert.to_pem().unwrap();

    /*faut implémenter le bon code avec la ACI !!!!!!!!!!!! */

    
    let path = format!("./new_certs_client/{}.pem", email);

    let mut file = File::create(&path).unwrap();
    file.write_all(&cert_pem).unwrap();

}