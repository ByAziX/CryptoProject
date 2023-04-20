#!/bin/bash

# Demander le nom de l'utilisateur pour la CSR
read -p "Entrez le nom de l'utilisateur : " username

# Demander le email de l'utilisateur pour la CSR
read -p "Entrez l'email de l'utilisateur : " email

# Créer un dossier pour l'utilisateur
mkdir $email
cd $email

# Générer une clé privée pour l'utilisateur
openssl genpkey -algorithm RSA -out private.key

# Créer une demande de certificat pour l'utilisateur
openssl req -new -key private.key -out $email.csr -subj "/CN=$username/emailAddress=$email"

# Afficher le contenu de la CSR
echo "Contenu de la CSR :"
cat $email.csr

# Afficher le contenu de la clé privée
echo "Contenu de la clé privée :"
cat private.key
