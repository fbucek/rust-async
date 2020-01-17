#!/usr/bin/env bash 

main() {
    bench
}


function bench() {

  info "Bench actix"

  cargo run --bin actix_async --release &
  sleep 2
  info "Bench actix ab"
  ab -r -n 100 -c 10 http://localhost:8080/
  info "Bench actix wrk"
  wrk -t10 -c100 -d1s http://localhost:8080/
  pkill actix_async

  info "Bench hyper"
  cargo run --bin hyper --release &
  sleep 2
  info "Bench hyper ab"
  ab -r -n 100 -c 10 http://localhost:3000/
  info "Bench hyper wrk"
  wrk -t10 -c100 -d1s http://localhost:3000/
  pkill hyper


}


# ################
# Output funcitons
# ################
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

function info () {
  echo -e "[${BLUE} $@ ${NC}]"
}

function pass() {
  echo -e "[${GREEN}PASS${NC}] $@"
}

function fail() {
  FAIL=1; echo -e "[${RED}FAIL${NC}] $@"
}

# Run main function
main "$@"