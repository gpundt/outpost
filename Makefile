SHELL := /bin/bash

## Colors ##
RED     := \033[0;31m
GREEN   := \033[0;32m
YELLOW  := \033[0;33m
BLUE    := \033[0;34m
CYAN	:= \033[0;36m
RESET   := \033[0m
define start_step_message
	@echo -e "\n$(CYAN)[*] $(1) [*]$(RESET)"
endef
define error_message
	@echo "$(RED)ERROR$(RESET): $(1)"
	$(error)
endef
define successful
	@echo -e "\t - $(GREEN)*Successful*$(RESET)\n"
endef

all: prep_provisioning_files

build_outpost:												## Builds individual rust binaries
	$(call start_step_message,"Building Outpost Binaries")
	$(MAKE) -f ./src/build.mk build
	$(call successful)

generate_certs:												## Generates server and client-side certificates
ifndef CLIENT_HOSTNAME
	$(call error_message,"CLIENT_HOSTNAME not defined")
endif
	cd ./scripts && ./generate_server_certs.sh
	cd ./scripts && ./generate_client_certs.sh $(CLIENT_HOSTNAME)

prep_provisioning_files: build_outpost generate_certs		## Prepares Ansible provisioning files
	$(call start_step_message,"Prepping Ansible Provisioning Directories")
	cp ./build/outpost_client ./provision/roles/outpost/files/outpost_client
	cp ./certs/ca/ca.crt ./provision/roles/outpost/files/ca.crt
	cp ./certs/server/server.* ./provision/roles/outpost/files/

help:														## Displays available make targets
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "$(BLUE)  %-30s$(RESET) %s\n", $$1, $$2}'