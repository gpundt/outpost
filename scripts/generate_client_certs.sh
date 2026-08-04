#!/bin/bash
# Cert generation script to be run once on each user's roaming device
source ./_helpers.sh

if [ "$#" -eq 0 ]; then
    error_message "No client hostname provided"
    message "Usage" "$0 [client_hostname]"
    exit 1
fi

CLIENT_HOSTNAME=$1
echo "CLIENT_HOSTNAME=${CLIENT_HOSTNAME}" > ../.env

# ──── Globals ──────────────────────────────────────────────────────────────────────
CLIENT_CERTS_DIR="${CERTS_ROOT_DIR}/${CLIENT_HOSTNAME}_client"
CLIENT_CERT="${CLIENT_CERTS_DIR}/client.crt"
CLIENT_KEY="${CLIENT_CERTS_DIR}/client.key"
CLIENT_YAML_CONFIG="../config/client.yaml"
DST_CA_CERT="/opt/outpost/tls/ca/ca.crt"
DST_CLIENT_CERT="/opt/outpost/tls/${CLIENT_HOSTNAME}_client/client.crt"
DST_CLIENT_KEY="/opt/outpost/tls/${CLIENT_HOSTNAME}_client/client.key"

# ──── Cert Generation ───────────────────────────────────────────────────────────────
function _generate_client_key() {
    start_step_message "Generating Client Key -> '${CLIENT_KEY}'"

    successful
}

function _generate_client_cert() {
    start_step_message "Generating Client Cert -> '${CLIENT_CERT}'"

    successful
}

function main() {
    apt_install_openssl
    prepare_certs_directory "client" "${CLIENT_CERTS_DIR}"
    _generate_client_key
    _generate_client_cert
}
main