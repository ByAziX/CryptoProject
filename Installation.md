# Rust Instllation with actix

cargo new hello-world
cd hello-world

##  Cargo.toml
[dependencies]
actix-web = "3.3.2"
serde = "1.0.130"
serde_derive = "1.0.130"
serde_json = "1.0.68"


# Générer une clé privée pour l'autorité de certification (CA) :
openssl ecparam -name secp384r1 -genkey -out racine.key

# Générer un certificat auto-signé pour l'ACR
openssl req -new -x509 -key racine.key -out racine.crt

# Configurer le fichier de configuration OpenSSL pour l'ACR

```
[ca]
default_ca = myCA

[myCA]
dir = ./racine
certs = $dir
new_certs_dir = $dir
database = $dir/index
serial = $dir/serial
private_key = ./racine.key
certificate = ./racine.crt
default_days = 365
default_crl_days = 30
default_md = sha256
policy = myCA_policy
copy_extensions = copyall
unique_subject = no

[myCA_policy]
countryName = optional
stateOrProvinceName = optional
organizationName = optional
organizationalUnitName = optional
commonName = supplied
emailAddress = optional

```
# Initialiser l'ACR

openssl ca -selfsign -config racine.cnf -extensions myCA_policy -in racine.crt -out racine.crt



# Générer une clé privée ECC pour l'autorité de certification intermédiaire : 

openssl ecparam -genkey -name secp384r1 -out intermediate-key.pem

# Créer une demande de certificat pour l'autorité de certification intermédiaire avec les extensions appropriées :
openssl req -new -key intermediate-key.pem -out intermediate-csr.pem -subj "/CN=MonAutoriteDeCertificationIntermediaire/O=MonOrganisation" -config <(echo -e "[req]\ndistinguished_name=dn\n[dn]\nCN=MonAutoriteDeCertificationIntermediaire\nO=MonOrganisation\n[ext]\nbasicConstraints=critical,CA:TRUE,pathlen:0\nkeyUsage=critical,keyCertSign,cRLSign\n")

# Signer la demande de certificat pour générer le certificat de l'autorité de certification intermédiaire :
openssl x509 -req -days 3650 -in intermediate-csr.pem -CA ca-cert.pem -CAkey ca-key.pem -CAcreateserial -out intermediate-cert.pem


# Sources

https://actix.rs/docs/getting-started


https://github.com/actix/examples/tree/master/forms/multipart