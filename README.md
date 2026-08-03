# outpost
Ansible deployment for offline, remote Linux hosts

# How to Deploy

To configure an outpost node, do the following:

1) Build the outpost Rust binaries:
    - `make`

2) Update the Ansible inventory with your desired Outpost node IPs:
    - `provision/inventory/hosts.yml`

3) Run the playbook:
    - `cd provision && ansible-playbook -i inventry/hosts.yml playbooks/outpost.yml`

4) Reboot each Outpost node, and you should see a new wireless network become available.

5) Connect to the new wireless network, and run the `outpost_client` binary:
    - `./outpost_client -h 192.168.99.1`