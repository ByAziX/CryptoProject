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

    let default_file = "Certificats/offline";
    let conf_file = "Certificats/offline/config/openssl.cnf";
    let output_file = "new_certs_client/";
    
    Command::new("openssl")
        .arg("ca")
        .arg("-keyfile")
        .arg(default_file.to_owned()+"/ACI/private.key")
        .arg("-cert")
        .arg(default_file.to_owned()+"/ACI/cacert.pem")
        .arg("-config")
        .arg(conf_file)
        .arg("-in")
        .arg(csr_file_path)
        .arg("-out")
        .arg(output_file.to_owned()+&email+".pem")
        .arg("-extensions")
        .arg("myCA_extensions")
        .arg("-batch")
        .output()
        .expect("Failed to execute command");

    


}