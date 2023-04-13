#!/bin/bash

# Demander le nom de l'utilisateur et le chemin vers sa CSR
read -p "Entrez le nom de l'utilisateur : " username
read -p "Entrez le chemin vers la CSR de l'utilisateur : " csr_path

# Vérifier l'adresse e-mail de l'utilisateur en lui envoyant un code à usage unique (à compléter selon votre méthode d'authentification)
# ...

# Vérifier le sujet de la CSR de l'utilisateur
if openssl req -text -noout -verify -in $csr_path >/dev/null 2>&1; then
    echo "La CSR de l'utilisateur $username a été vérifiée avec succès."
else
    echo "Erreur : la CSR de l'utilisateur $username n'est pas valide."
    exit 1
fi

# Demander la génération du certificat à l'ACI
echo "Demande de génération du certificat de l'utilisateur $username à l'ACI en cours..."
openssl ca -in $csr_path -config ACI/openssl.cnf -extensions myuser_extensions -batch

# Mettre à disposition de l'utilisateur le certificat, ainsi que ceux de l'ACR et de l'ACI
echo "Certificat généré pour l'utilisateur $username."
cp ACI/newcerts/* $username/
cp ACR/cacert.pem $username/
cp ACI/cacert.pem $username/
