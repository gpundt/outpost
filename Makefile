SHELL := /bin/bash

all: build_outpost

build_outpost:					## Builds individual rust binaries
	$(MAKE) -C ./src build

help:							## Displays available make targets
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "$(BLUE)  %-30s$(RESET) %s\n", $$1, $$2}'