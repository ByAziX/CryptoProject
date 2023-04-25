# Rust Instllation with actix

cargo new hello-world
cd hello-world
cargo watch -x run

# lire un certificat 
openssl x509 -in ./ISEN/Cours/Cryptographie/CryptoWebsiteCA/CryptoProject/new_certs_client/hugo.millet@isen.yncrea.fr.pem -text -noout


# verifier les certificat entre la ca et la aci :

openssl verify -CAfile ./ACR/cacert.pem ./ACI/cacert.pem

# verifier certificat pour l'utilisateur
openssl verify -CAfile ./ACR/cacert.pem -untrusted ./ACI/cacert.pem ../../new_certs_client/hugo.millet@isen.yncrea.fr.pem 


# CLR rvocation : 
openssl ca -config <config_file> -revoke <certificate_file>
openssl ca -config <config_file> -gencrl -out <crl_file>


# oscp

https://bhashineen.medium.com/create-your-own-ocsp-server-ffb212df8e63*


# Start OCSP Server. Switch to a new terminal and run

openssl ocsp -index index -port 8888 -rsigner ../oscp/ocspSigning.crt -rkey ../oscp/ocspSigning.key -CA cacert.pem -text -out ../oscp/log.txt &

# Verify Certificate Revocation. Switch to a new terminal and run

openssl ocsp -CAfile cacert.pem -issuer ../ACR/cacert.pem -cert ../../../new_certs_client/hugo.millet@isen.yncrea.fr.pem -url http://127.0.0.1:8888 -resp_text -noverify

# revoke
openssl ca -keyfile private.key -cert cacert.pem -revoke ../../../new_certs_client/hugo.millet@isen.yncrea.fr.pem -config config/openssl.cnf



# Then restart the OCSP server.

openssl ocsp -index index -port 8888 -rsigner ../oscp/ocspSigning.crt -rkey ../oscp/ocspSigning.key -CA cacert.pem -text -out ../oscp/log.txt &

# Verify Certificate Revocation. Switch to a new terminal and run

openssl ocsp -CAfile cacert.pem -issuer ../ACR/cacert.pem -cert ../../../new_certs_client/hugo.millet@isen.yncrea.fr.pem -url http://127.0.0.1:8888 -resp_text -noverify





# probleme avec openssl install : 

sudo apt-get install libssl-dev pkg-config

# création server smtp gmail 

https://www.youtube.com/watch?v=g_j6ILT-X0k

# password smtp
dqvjnxkzwdjdoktc

https://myaccount.google.com/u/1/apppasswords?pli=1&rapt=AEjHL4Ob_b4z4AZocvzwoV2WouTtfxrULkI---gSo_vJMwBJS3DWGz0MdpfpFOnTjmdmiOBG9HHLcR9gt53ZKHLfEuShWnO1JA

##  Cargo.toml
[dependencies]
actix-web = "3.3.2"
serde = "1.0.130"
serde_derive = "1.0.130"
serde_json = "1.0.68"


# Générer une clé privée pour l'autorité de certification (CA) :

https://gist.github.com/Soarez/9688998

openssl ecparam -name secp384r1 -genkey -out ca.key

# Generate a CSR
openssl req -new -key ca.key -out ca.csr


openssl ca -config ca.cnf -out ca.crt -infiles ca.csr




# ca.cnf --------------------->

# we use 'ca' as the default section because we're usign the ca command
[ ca ]
default_ca = my_ca

[ my_ca ]
#  a text file containing the next serial number to use in hex. Mandatory.
#  This file must be present and contain a valid serial number.
serial = ./serial

# the text database file to use. Mandatory. This file must be present though
# initially it will be empty.
database = ./index.txt

# specifies the directory where new certificates will be placed. Mandatory.
new_certs_dir = ./newcerts

# the file containing the CA certificate. Mandatory
certificate = ./ca.crt

# the file contaning the CA private key. Mandatory
private_key = ./ca.key

# the message digest algorithm. Remember to not use MD5
default_md = sha256

# for how many days will the signed certificate be valid
default_days = 365

# a section with a set of variables corresponding to DN fields
policy = my_policy

[ my_policy ]
# if the value is "match" then the field value must match the same field in the
# CA certificate. If the value is "supplied" then it must be present.
# Optional means it may be present. Any fields not mentioned are silently
# deleted.
countryName = match
stateOrProvinceName = supplied
organizationName = supplied
commonName = supplied
organizationalUnitName = optional
commonName = supplied

# specify the path length constraint for the CA's issued certificates
pathlen = 3



# créate new directory, index , serial file
mkdir newcerts
touch index.txt
echo '01' > serial

# sign the certificate

openssl ca -config ca.cnf -out ca.crt -infiles ca.csr


openssl x509 -in oats.crt -noout -text







# ACI

openssl ecparam -name secp384r1 -genkey -out intermediate.key
openssl req -new -key intermediate.key -out intermediate.csr


openssl ca -in ../ACI/intermediate.csr -out ../ACI/intermediate.crt -cert ca.crt -keyfile ca.key -config ca.cnf




# Sources

https://actix.rs/docs/getting-started


https://github.com/actix/examples/tree/master/forms/multipart