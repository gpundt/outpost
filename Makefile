SHELL := /bin/bash

## Colors ##
RED     := \033[0;31m
GREEN   := \033[0;32m
YELLOW  := \033[0;33m
BLUE    := \033[0;34m
CYAN	:= \033[0;36m
RESET   := \033[0;0m
define start_step_message
	@echo -e "\n$(CYAN)[*] $(1) [*]$(RESET)"
endef
define error_message
	@echo -e "$(RED)ERROR$(RESET): $(1)"
	$(error $(RED)ERROR$(RESET): $(1))
endef
define successful
	@echo -e "\t - $(GREEN)*Successful*$(RESET)\n"
endef

all: generate_certs build_outpost prep_provisioning_files

PIZERO_HOST      ?= mesh@192.168.1.191
PIZERO_TARGET    := arm-unknown-linux-gnueabihf
PIZERO_TOOLCHAIN ?= $(HOME)/opt/x-tools/armv6-rpi-linux-gnueabihf/bin
PIZERO_SYSROOT   := $(CURDIR)/outpost_server/sysroots/pizero

PLATFORM ?= native

prepare_build_output_dir:
	@mkdir -p ./build

sysroot-pizero:												## Pulls libudev/libc/etc from a real Pi Zero W (override with PIZERO_HOST=user@ip)
	$(call start_step_message,"Syncing Pi Zero W sysroot from $(PIZERO_HOST)")
	@test -f ./scripts/sync_pizero_sysroot.sh || { echo "missing scripts/sync_pizero_sysroot.sh"; exit 1; }
	@cd scripts && ./sync_pizero_sysroot.sh $(PIZERO_HOST) $(PIZERO_SYSROOT)
	$(call successful)

sysroot-pizero-refresh:										## Forces a clean re-sync of the Pi Zero W sysroot
	@rm -rf $(PIZERO_SYSROOT)
	$(MAKE) sysroot-pizero

build_outpost_server_pizero: sysroot-pizero					## Cross-compiles outpost_server for the Pi Zero W (ARMv6)
	$(call start_step_message,"Cross-compiling outpost_server for Pi Zero W")
	rustup target add arm-unknown-linux-gnueabihf && \
	cd outpost_server && \
	PATH="$(PIZERO_TOOLCHAIN):$$PATH" \
	PKG_CONFIG_ALLOW_CROSS=1 \
	PKG_CONFIG_LIBDIR="$(PIZERO_SYSROOT)/usr/lib/arm-linux-gnueabihf/pkgconfig" \
	PKG_CONFIG_SYSROOT_DIR="$(PIZERO_SYSROOT)" \
	CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=armv6-rpi-linux-gnueabihf-gcc \
	CC_arm_unknown_linux_gnueabihf=armv6-rpi-linux-gnueabihf-gcc \
	CXX_arm_unknown_linux_gnueabihf=armv6-rpi-linux-gnueabihf-g++ \
	AR_arm_unknown_linux_gnueabihf=armv6-rpi-linux-gnueabihf-ar \
	RUSTFLAGS="-C link-arg=--sysroot=$(PIZERO_SYSROOT)" \
	cargo build --release --target $(PIZERO_TARGET) -p outpost_server
	cp outpost_server/target/$(PIZERO_TARGET)/release/outpost_server ./build/outpost_server_pizero
	$(call successful)

build_outpost_server_native:								## Builds outpost_server for whatever machine you're running make on (e.g. Windows)
	$(call start_step_message,"Building outpost_server for host platform")
	cd outpost_server && \
	unset PKG_CONFIG_LIBDIR PKG_CONFIG_SYSROOT_DIR PKG_CONFIG_ALLOW_CROSS \
	      CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER \
	      CC_arm_unknown_linux_gnueabihf CXX_arm_unknown_linux_gnueabihf AR_arm_unknown_linux_gnueabihf \
	      RUSTFLAGS && \
	cargo build --release -p outpost_server
	cp outpost_server/target/release/outpost_server ./build/outpost_server_native
	$(call successful)

build_outpost_client:										## Builds outpost_client
	$(call start_step_message,"Building outpost_client")
	cd outpost_client && cargo build --release -p outpost_client
	cp outpost_client/target/release/outpost_client ./build/outpost_client
	$(call successful)

build_outpost: 												## Builds outpost binaries. Set PLATFORM=pizero to cross-compile for the Pi Zero W
	$(MAKE) prepare_build_output_dir							
ifeq ($(PLATFORM),pizero)
	$(MAKE) build_outpost_server_pizero
else
	$(MAKE) build_outpost_server_native
endif
	$(MAKE) build_outpost_client
	$(call successful)

generate_certs:												## Generates server and client-side certificates
	$(call start_step_message,"Generating server and client certificates")
ifndef CLIENT_HOSTNAME
	$(call error_message,"CLIENT_HOSTNAME not defined")
endif
	@cd ./scripts && ./generate_server_certs.sh
	@cd ./scripts && ./generate_client_certs.sh $(CLIENT_HOSTNAME)

prep_provisioning_files: 									## Prepares Ansible provisioning files
	$(call start_step_message,"Prepping Ansible Provisioning Directories")
ifndef AP_SSID
	$(call error_message,"AP_SSID is not specified")
else
	sed -i 's/^ssid=.*/ssid=$(AP_SSID)/' ./provision/roles/outpost/files/hostapd.conf
endif
ifndef AP_PASSWORD
	$(call error_message,"AP_PASSWORD flag is not specified")
else
	sed -i 's/^wpa_passphrase=.*/wpa_passphrase=$(AP_PASSWORD)/' ./provision/roles/outpost/files/hostapd.conf
endif
ifdef AP_INTERFACE
	sed -i 's/^interface=.*/interface=$(AP_INTERFACE)/' ./provision/roles/outpost/files/hostapd.conf
	sed -i 's/^interface=.*/interface=$(AP_INTERFACE)/' ./provision/roles/outpost/files/dnsmasq.conf
else
	sed -i 's/^interface=.*/interface=wlan0/' ./provision/roles/outpost/files/hostapd.conf
	sed -i 's/^interface=.*/interface=wlan0/' ./provision/roles/outpost/files/dnsmasq.conf
endif
	cp ./build/outpost_server* ./provision/roles/outpost/files/
	cp ./certs/ca/ca.crt ./provision/roles/outpost/files/ca.crt
	cp ./certs/server/server.* ./provision/roles/outpost/files/
	$(call successful)

help:														## Displays available make targets
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "$(BLUE)  %-30s$(RESET) %s\n", $$1, $$2}'