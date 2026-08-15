#!/bin/bash
source ./_helpers.sh
set -euo pipefail

PIZERO_HOST="${1:?Usage: $0 <user@host> <sysroot-dest-dir>}"
DEST="${2:?Usage: $0 <user@host> <sysroot-dest-dir>}"

function prepare_sysroot_dirs() {
    start_step_message "Preparing Output Directories '${DEST}'"
    mkdir -p "${DEST}/lib/arm-linux-gnueabihf"
    mkdir -p "${DEST}/usr/lib/arm-linux-gnueabihf/pkgconfig"
    mkdir -p "${DEST}/usr/include"
    successful
}

function pull_pizero_libs() {
    start_step_message "Pulling Required Libs from ${PIZERO_HOST}'"
    if ! rsync -avzL "${PIZERO_HOST}:/lib/ld-linux-armhf.so.3"      "${DEST}/lib/"; then
        error_message "Rsync Failed" "exit"
    fi
    if ! rsync -avzL "${PIZERO_HOST}:/lib/arm-linux-gnueabihf/"      "${DEST}/lib/arm-linux-gnueabihf/"; then
        error_message "Rsync Failed" "exit"
    fi
    if ! rsync -avzL "${PIZERO_HOST}:/usr/lib/arm-linux-gnueabihf/"  "${DEST}/usr/lib/arm-linux-gnueabihf/"; then
        error_message "Rsync Failed" "exit"
    fi
    if ! rsync -avzL "${PIZERO_HOST}:/usr/include/"                  "${DEST}/usr/include/"; then
        error_message "Rsync Failed" "exit"
    fi
    successful
}

function main() {
    prepare_sysroot_dirs
    pull_pizero_libs
}
main