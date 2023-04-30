# Rust install

cargo new hello-world
cd hello-world

# Rust run
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


# Start OCSP Server. Switch to a new terminal and run

openssl ocsp -index index -port 8888 -rsigner ../ocsp/ocspSigning.crt -rkey ../ocsp/private.key -CA cacert.pem -text -out ../ocsp/log.txt &

# Verify Certificate Revocation. Switch to a new terminal and run

openssl ocsp -CAfile cacert.pem -issuer cacert.pem -cert ./newcerts/03.pem -url http://127.0.0.1:8888 -resp_text -noverify

# revoke
openssl ca -keyfile private.key -cert cacert.pem -revoke ./newcerts/03.pem -config config/openssl.cnf                                                                                                                                   


# password smtp
dqvjnxkzwdjdoktc

https://myaccount.google.com/u/1/apppasswords?pli=1&rapt=AEjHL4Ob_b4z4AZocvzwoV2WouTtfxrULkI---gSo_vJMwBJS3DWGz0MdpfpFOnTjmdmiOBG9HHLcR9gt53ZKHLfEuShWnO1JA


# Sources

https://actix.rs/docs/getting-started

https://github.com/actix/examples/tree/master/forms/multipart

https://gist.github.com/Soarez/9688998

https://www.youtube.com/watch?v=g_j6ILT-X0k

https://bhashineen.medium.com/create-your-own-ocsp-server-ffb212df8e63