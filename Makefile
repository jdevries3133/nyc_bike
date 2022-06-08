SHELL:=/bin/bash


.PHONY: develop
develop:
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
