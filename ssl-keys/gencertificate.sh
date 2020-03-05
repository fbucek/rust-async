#!/usr/bin/env bash

function info () {
    echo -e "[\033[0;34m $@ \033[0m]"
}

SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd $SRCDIR

NAME=rustasync

#echo "Creating certificates"
#openssl req -x509 -out $NAME.crt -keyout $NAME.key \
#   -newkey rsa:2048 -nodes -sha256 \
#   -subj '/CN=rustasync' -extensions EXT -config <( \
#   printf "[dn]\nCN=rustasync\n[req]\ndistinguished_name = dn\n[EXT]\nsubjectAltName=DNS:localhost\nkeyUsage=digitalSignature\nextendedKeyUsage=serverAuth")

info "Deleting certificat $NAME from system keychain"
sudo security delete-certificate -c $NAME
sudo security delete-certificate -c localhost
sudo security remove-trusted-cert -d $NAME

if [ ! -z $1 ] && [[ "$1" = "generate" ]];then 
    info "Creating certificates"
    openssl req -new -extensions EXT -config rustasync.config -x509 -out $NAME.crt -keyout $NAME.key
fi

info "Installing $NAME.crt to system keychain"
sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain $NAME.crt
