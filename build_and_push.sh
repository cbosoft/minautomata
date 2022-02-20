#!/usr/bin/env bash

ALRIGHT_THEN="$(cat ~/.alright_then)"
FTP_DEST="$(cat ~/.ftp_dest)"
TARGET="minautomata"

wasm-pack build --target web || exit 1

mkdir -p "${TARGET}" || exit 1

cp -r pkg *.js *.html *.css "${TARGET}/." || exit 1

find "${TARGET}" -type f -exec curl -u "cmjbteo:${ALRIGHT_THEN}" --ftp-create-dirs -T {} ftp://"${FTP_DEST}"/\{\} \;
