#!/usr/bin/env bash

SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

cd $SRCDIR

NAME=rustasync

CONFIG="[dn]\nCN=localhost\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:localhost\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth"

echo "Creating certificates"
openssl req -x509 -out $NAME.crt -keyout $NAME.key \
    -newkey rsa:2048 -nodes -sha256 \
    -subj '/CN=rustasync' -extensions EXT -config <( \
    printf "[dn]\nCN=rustasync\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:localhost\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth")

echo "Deleting certificat $NAME from system keychain"
sudo security delete-certificate -c $NAME

echo "Installing $NAME.crt to system keychain"
sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain $NAME.crt
