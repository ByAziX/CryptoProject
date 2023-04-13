#!/bin/bash

# Demander le nom de l'utilisateur pour la CSR
read -p "Entrez le nom de l'utilisateur : " username

# Créer un dossier pour l'utilisateur
mkdir $username
cd $username

# Générer une clé privée pour l'utilisateur
openssl genpkey -algorithm RSA -out private.key

# Créer une demande de certificat pour l'utilisateur
openssl req -new -key private.key -out $username.csr -subj "/CN=$username"

# Afficher le contenu de la CSR
echo "Contenu de la CSR :"
cat $username.csr

# Afficher le contenu de la clé privée
echo "Contenu de la clé privée :"
cat private.key
