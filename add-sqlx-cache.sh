#!/bin/bash
run_sqlx_prepare_if_necessary() {
    local dir=$1
    local changes

    changes=$(git diff --cached --name-only | grep "^$dir/")

    if [ -n "$changes" ]; then
        echo "Changes detected in $dir. Running cargo sqlx prepare..."
        (cd "$dir" && cargo sqlx prepare)
        git add "$dir/.sqlx/query-*.json"
    else
        echo "No changes detected in $dir."
    fi
}

run_sqlx_prepare_if_necessary "flights-monitor"
run_sqlx_prepare_if_necessary "flights-web"
