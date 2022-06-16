SHELL:=/bin/bash


.PHONY: develop
develop:
	# flutter CLI is the only one that uses STDIN normally (`r` to reload)
	concurrently \
		-n web_front,mobile,backend \
		"make --directory=./web_front develop" \
		"make --directory=./mobile    develop" \
		"make --directory=./backend   develop"


.PHONY: deploy
deploy:
	make --directory=./backend deploy
	make --directory=./web_front deploy
	make --directory=./doc deploy


.PHONY: check
check:
	cargo test
	cargo fmt --check


.PHONY: setup-ci
setup-ci:
	make --directory=./backend setup-ci
	make --directory=./web_front setup-ci
