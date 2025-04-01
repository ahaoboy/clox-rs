#!/bin/bash

REPO_URL="https://github.com/munificent/craftinginterpreters.git"
REPO_DIR="craftinginterpreters"
PATCH_FILE="0001-c2rust.patch"
TARGET_DIR="c"

if [ ! -d "$REPO_DIR" ]; then
    git clone --depth=1 "$REPO_URL" "$REPO_DIR"
    cd "$REPO_DIR"
    git am "../$PATCH_FILE"
    cd ..
    cp -r "$REPO_DIR/c" "./$TARGET_DIR"
    rm -r ./c/clox.xcodeproj
fi

node update.js

c2rust transpile ./clox-one.c