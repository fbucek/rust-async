#!/usr/bin/env bash 


# Simple unvalid benchmark just to test if there is big difference or not
# Number does reflect production use

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
  sleep 1

  info "Bench hyper"
  cargo run --bin hyper --release &
  sleep 2
  info "Bench hyper ab"
  ab -r -n 100 -c 10 http://localhost:3000/
  info "Bench hyper wrk"
  wrk -t10 -c100 -d1s http://localhost:3000/
  pkill hyper
  sleep 1

  info "Bench warp"
  cargo run --bin warp --release &
  sleep 2
  info "Bench warp ab"
  ab -r -n 100 -c 10 http://localhost:3030/
  info "Bench warp wrk"
  wrk -t10 -c100 -d1s http://localhost:3030/
  pkill warp
  sleep 1

  info "Bench warpssl"
  cd warpssl
  cargo run --bin warpssl --release &
  sleep 2
  info "Bench warpssl ab"
  ab -r -n 100 -c 10 https://localhost:3030/
  info "Bench warpssl wrk"
  wrk -t10 -c100 -d1s https://localhost:3030/
  pkill warpssl
  cd ..


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