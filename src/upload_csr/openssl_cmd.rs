extern crate openssl;
use openssl::x509::X509Req;
use openssl::asn1::Asn1String;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;


pub(crate) async fn get_csr_subject(csr_file_path: &str) -> Result<String, String> {
    // Lire le fichier CSR
    let mut file = File::open(csr_file_path).map_err(|e| format!("Erreur lors de l'ouverture du fichier CSR : {}", e))?;
    let mut csr_bytes = Vec::new();
    file.read_to_end(&mut csr_bytes).map_err(|e| format!("Erreur lors de la lecture du fichier CSR : {}", e))?;

    // Charger le CSR en tant qu'objet X509Req
    let csr = X509Req::from_pem(&csr_bytes).map_err(|e| format!("Erreur lors de la lecture du CSR : {}", e))?;

    //get emailAddress from CSR with csr

    let email = csr.subject_name().entries_by_nid(openssl::nid::Nid::PKCS9_EMAILADDRESS).next().unwrap().data().as_utf8().unwrap();


/*
    // Obtenir le sujet du CSR
    let subject = csr.subject_name();
    let subject = subject.entries_by_nid(openssl::nid::Nid::COMMONNAME).next().unwrap().data().as_utf8().unwrap();*/
    Ok(email.to_string())
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

    /*let mut keyFile = File::open("/home/hugo/ISEN/Cours/Cryptographie/CryptoWebsiteCA/CryptoProject/Certificats/offline/ACI/private.key").unwrap();
    let mut contents = String::new();
    keyFile.read_to_string(&mut contents).unwrap();  
    let key = openssl::pkey::PKey::private_key_from_pem(contents.as_bytes()).unwrap();

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
    // CA false
    
    builder.sign(&key, openssl::hash::MessageDigest::sha256()).unwrap();

    let cert = builder.build();

    let cert_pem = cert.to_pem().unwrap();
    */

    /*faut implémenter le bon code avec la ACI !!!!!!!!!!!! */

    let conf_file = "/home/hugo/ISEN/Cours/Cryptographie/CryptoWebsiteCA/CryptoProject/Certificats/offline/config/openssl.cnf";
    let output_days = "90";
    
    let cert_pem = Command::new("openssl")
        .arg("ca")
        .arg("-batch")
        .arg("-config")
        .arg(conf_file)
        .arg("-in")
        .arg(csr_file_path)
        .arg("-days")
        .arg(output_days)
        .output()
        .expect("Failed to execute command");

    
    log::info!("cert_pem: {:?}", cert_pem);
    
    let path = format!("./new_certs_client/{}.pem", email);

    let mut file = File::create(&path).unwrap();
    file.write_all(&cert_pem.stdout).unwrap();

}