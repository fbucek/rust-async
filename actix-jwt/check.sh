#!/usr/bin/env bash

# src: https://gist.github.com/fbucek/f986da3cc3a9bbbd1573bdcb23fed2e1
set -e # error -> trap -> exit
function info() { echo -e "[\033[0;34m $@ \033[0m]"; } # blue: [ info message ]
function pass() { echo -e "[\033[0;32mPASS\033[0m] $@"; } # green: [PASS]
function fail() { FAIL="true"; echo -e "[\033[0;31mFAIL\033[0m] $@"; } # red: [FAIL]
trap 'LASTRES=$?; LAST=$BASH_COMMAND; if [[ LASTRES -ne 0 ]]; then fail "Command: \"$LAST\" exited with exit code: $LASTRES"; elif [ "$FAIL" == "true"  ]; then fail finished with error; else echo -e "[\033[0;32m Finished $@ \033[0m]";fi' EXIT
SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )" # this source dir


info "Sleep for until port is open"
while ! nc -z localhost 8080; do   
    sleep 0.1 # wait for 1/10 of the second before check again
done

info "Get user"
curl -X GET 'http://127.0.0.1:8080/users/45'



info "Sign up"
curl -X POST -i 'http://127.0.0.1:8080/api/auth/signup' \
    -H "Content-Type: application/json" \
    --data '{
    "username": "user",
    "email": "user@email.com",
    "password": "4S3cr3tPa55w0rd"
    }'


