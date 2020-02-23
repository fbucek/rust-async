#!/usr/bin/env bash

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


# info "direct curl -> api.github.com"
# curl https://api.github.com/users/octocat/orgs

info "monitorsdk hyper 1 proxy -> api.github.com"
# curl -i https://localhost:3030/users/octocat/orgs
curl -i -H "Accept: application/json" -H "Content-Type: application/json" "https://localhost:3030/users/octocat/orgs"

# info "AWS monitorsdk proxy -> api.github.com"
# curl http://ec2-3-120-98-176.eu-central-1.compute.amazonaws.com/users/octocat/orgs





# curl https://gitlab.com/gitlab-org/gitlab-foss/issues/62077
# curl https://localhost:8090/gitlab-org/gitlab-foss/issues/62077
