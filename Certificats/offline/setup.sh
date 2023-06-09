#!/bin/bash

originalFolder=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

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
  mkdir -p ACR/config
  cd ACR
  # Générer une clé privée pour l'ACR

  openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:prime256v1 -out private.key

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

  mkdir -p ACI/config
  cd ACI
  # Générer une clé privée pour l'ACR

  openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:prime256v1 -out private.key

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

oscp(){
  currentFolder="$originalFolder/ocsp"
  configFolder="$originalFolder/config"

  cd $originalFolder

  if [ -d "oscp" ]; then
    # delete the ACR folder
    rm -rf oscp

  fi

  mkdir ocsp 
  mkdir -p ocsp/config

  cd ocsp 

   openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:prime256v1 -out private.key


   cp "$configFolder/ocsp.cnf" "$currentFolder/config/openssl.cnf"
   echo "creation de l'oscp"

  openssl req -new -key private.key -out csr.pem -config "$currentFolder/config/openssl.cnf"


  cd ../ACI/

  openssl ca -keyfile private.key -cert cacert.pem -in ../ocsp/csr.pem -out ../ocsp/ocspSigning.crt -config "$currentFolder/config/openssl.cnf" -batch

}

create_root_ca
cd $originalFolder
# print a blank line
echo "Root CA created"
echo ""
echo ""
create_intermediate_ca
oscp