#!/bin/bash

originalFolder="/home/hugo/ISEN/Cours/Cryptographie/CryptoWebsiteCA/CryptoProject/Certificats/offline"
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

  openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:secp384r1 -out private.key


  # Créer un fichier de configuration pour l'ACR
  cp "$configFolder/ACR.cnf" "$currentFolder/config/openssl.cnf"
  echo "creation de l'ACR"
  # Créer une demande de certificat pour l'ACR
  openssl req -new -key private.key -out csr.pem -config "$currentFolder/config/openssl.cnf"

  echo "creation des dossiers"
  mkdir newcerts
  touch index serial && echo "01" >serial
  # Signer la demande de certificat avec elle-même pour créer le certificat de l'ACR
  openssl ca -selfsign -keyfile private.key -config "$currentFolder/config/openssl.cnf" -in csr.pem -out cacert.pem -extensions myCA_extensions -batch
  # Copier les fichiers cacert.pem et private.key dans un dossier sécurisé
  cp cacert.pem $secureFolder
  cp private.key $secureFolder
}

create_intermediate_ca() {
  currentFolder="$originalFolder/ACI"
  secureFolder="$originalFolder/secure/"
  configFolder="$originalFolder/config"

  # Verify that ACR does not already exist
  if [ -d "ACI" ]; then
    # delete the ACR folder
    rm -rf ACI

  fi

  # Créer un dossier pour stocker les fichiers de l'ACR

  mkdir ACI
  cd ACI
  mkdir config

  # Générer une clé privée pour l'ACR

  openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:secp384r1 -out private.key

  cp "$configFolder/ACI.cnf" "$currentFolder/config/openssl.cnf"
  echo "creation de l'ACI"
  # Créer une demande de certificat pour l'ACR
  openssl req -new -key private.key -out csr.pem -config "$currentFolder/config/openssl.cnf"

  mkdir newcerts
  touch index serial && echo "01" >serial

  # Signer la demande de certificat avec l'ACR pour créer le certificat de l'ACI
  openssl ca -keyfile "$originalFolder/ACR/private.key" -cert "$originalFolder/ACR/cacert.pem" -config "$originalFolder/config/ACI.cnf" -in csr.pem -out "$currentFolder/cacert.pem" -extensions myCA_extensions -batch

  # Copier les fichiers cacert.pem et private.key dans un dossier sécurisé
  cp "$currentFolder/cacert.pem" $secureFolder
  cp "$currentFolder/private.key" $secureFolder
}

create_root_ca
cd $originalFolder
# print a blank line
echo "Root CA created"
echo ""
echo ""
create_intermediate_ca
