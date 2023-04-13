#!/bin/bash

originalFolder="/home/thomas/Documents/Education/M1/crypto/projet/offline"
cd $originalFolder

create_root_ca() {
  currentFolder="$originalFolder/ACR"
  secureFolder="$originalFolder/secure/"
  configFolder="$originalFolder/config"

  # Verify that ACR does not already exist
  if [ -d "ACR" ]; then
    # delete the ACR folder
    rm -rf ACR

  fi

  # Créer un dossier pour stocker les fichiers de l'ACR
  mkdir ACR
  cd ACR
  mkdir config

  # Générer une clé privée pour l'ACR
  openssl genpkey -algorithm RSA -out private.key

  # Créer un fichier de configuration pour l'ACR
  cp "$configFolder/ACR.cnf" "$currentFolder/config/openssl.cnf"
  echo "creation de l'ACR"
  # Créer une demande de certificat pour l'ACR
  openssl req -new -key private.key -out csr.pem -config "$currentFolder/config/openssl.cnf"

  echo "creation des dossiers"
  mkdir newcerts
  touch index serial && echo "01" >serial
  # Signer la demande de certificat avec elle-même pour créer le certificat de l'ACR
  openssl ca -selfsign -keyfile private.key -config "$currentFolder/config/openssl.cnf" -in csr.pem -out cacert.pem -extensions myCA_extensions -days 365 -batch
  # Copier les fichiers cacert.pem et private.key dans un dossier sécurisé
  cp cacert.pem $secureFolder
  cp private.key $secureFolder
}

create_intermediate_ca() {
  currentFolder="$originalFolder/ACI"
  secureFolder="$currentFolder/secure/"
  configFolder="$currentFolder/config"

  # Verify that ACI does not already exist
  if [ -d "ACI" ]; then
    # delete the ACI folder
    rm -rf ACI

  fi
  # Créer un dossier pour stocker les fichiers de l'ACI
  mkdir ACI
  cd ACI
  mkdir secure config
  # Générer une clé privée pour l'ACI
  openssl genpkey -algorithm RSA -out private.key

  cp "$originalFolder/config/ACI.cnf" "$currentFolder/config/openssl.cnf"
  # Créer une demande de certificat pour l'ACI
  openssl req -new -key private.key -out csr.pem -config "$currentFolder/config/openssl.cnf"

  mkdir newcerts && touch index serial && echo "01" >serial

  # Signer la demande de certificat avec l'ACR pour créer le certificat de l'ACI
  openssl ca -keyfile "$originalFolder/secure/private.key" -cert "$originalFolder/secure/cacert.pem" -config "$originalFolder/config/ACR.cnf" -in csr.pem -out cacert.pem -extensions myCA_extensions -days 365 -batch

  # Copier les fichiers cacert.pem et private.key dans un dossier sécurisé
  cp cacert.pem $secureFolder
  cp private.key $secureFolder
}

create_root_ca
cd $originalFolder
# print a blank line
echo "Root CA created"
echo ""
echo ""
create_intermediate_ca
