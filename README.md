# outpost
Ansible-deployed Meshtastic connector for offline, remote Linux hosts

# How to Deploy

To configure an outpost server, do the following:

1) Build the outpost Rust binaries:
    - `make AP_SSID=<WIRELESS AP NAME> AP_PASSWORD=<WIRELESS AP PASSWORD>`

2) Update the Ansible inventory with your desired Outpost node IPs:
    - `provision/inventory/hosts.yml`

3) Create an Ansible vault with each outpost node's SSH password and sudo password:
```bash
cd provision
ansible-vault create vault.yml
...
vault_ssh_user:  "<NODE_SSH_USER>"
vault_ssh_password: "<NODE_SSH_PASSWORD>"
vault_become_password: "<NODE_SUDO_PASSWORD>"
```

4) Run the playbook:
    - `ansible-playbook -i inventry/hosts.yml outpost.yml --ask-vault-pass`

5) Review output for any errors and troubleshoot accordingly

6) Reboot each Outpost node, and you should see a new wireless network become available.

7) Connect to the new wireless network, and run the `outpost_client` binary:
    - `./outpost_client -h 192.168.99.1`