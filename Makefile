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
	make --directory=./web_front setup

.PHONY: develop
develop: setup
	make --directory=./web_front develop


.PHONY: deploy
deploy:
	make --directory=./docs deploy
	make --directory=./web_front deploy
