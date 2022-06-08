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
	# flutter CLI is the only one that uses STDIN normally (`r` to reload)
	concurrently \
		-n web_front,backend,mobile \
		"make --directory=./web_front develop" \
		"make --directory=./mobile    develop" \
		"make --directory=./backend   develop"


.PHONY: deploy
deploy:
	make --directory=./backend deploy
	make --directory=./web_front deploy
	make --directory=./docs deploy


.PHONY: check
check:
	make --directory=./web_front check
	make --directory=./backend check
