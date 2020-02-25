#!/usr/bin/env bash

function info () {
    echo -e "[\033[0;34m $@ \033[0m]"
}

SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd $SRCDIR

NAME=rustasync

info "Creating certificates"
openssl req -new -subj '/CN=rustasync' -config rustasync.config -x509 -out $NAME.crt -keyout $NAME.key

info "Deleting certificat $NAME from system keychain"
sudo security delete-certificate -c $NAME

info "Installing $NAME.crt to system keychain"
sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain $NAME.crt
