#!/usr/bin/env bash

NAME=hyper-localhost

echo "Creating certificates"
openssl req -x509 -out keys/$NAME.crt -keyout keys/$NAME.key \
    -newkey rsa:2048 -nodes -sha256 \
    -subj '/CN=hyper-localhost' -extensions EXT -config <( \
    printf "[dn]\nCN=hyper-localhost\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:localhost\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth")

echo "Deleting certificat $NAME from system keychain"
sudo security delete-certificate -c $NAME

echo "Installing $NAME.crt to system keychain"
sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain keys/$NAME.crt