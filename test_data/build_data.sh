#!/bin/bash

function build {
    echo "$1 => $2"
    output="$(pwd)/$2"
    if [ ! -d "$ouput" ]; then
        mkdir -pv "$output"
    fi;
    pushd "$1" > /dev/null
    RUSTFLAGS=-Zsave-analysis cargo build
    cp target/debug/deps/save-analysis/*.json "$output"
    # strip all hashes from filenames libfoo-[hash].json -> libfoo.json
    for from in $output/*.json; do
        to=$(echo "$from" | sed -e "s/\(.*\)-[a-f0-9]*.json/\1.json/1")
        mv "$from" "$to"
    done
    popd > /dev/null
}

build ogl ogl/save-analysis
build hello hello/save-analysis
