SHELL:=/bin/bash

.PHONY: setup
setup:
	@# check installation of npm package `concurrently` which is used in other
	@# makefiles
	@which concurrently > /dev/null; \
	if [[ $$? -ne 0 ]]; then \
		echo "fatal: must install concurrently with ``npm install --global concurrently``"; \
		exit 1; \
	fi

	cd web_front && make setup

.PHONY: develop
develop: setup
	concurrently \
		"cd web_front && make develop"
