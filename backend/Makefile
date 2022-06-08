SHELL=/bin/bash

DOCKER_ACCOUNT=jdevries3133
CONTAINER_NAME=rust_template_backend

TAG?=$(shell git describe --tags)
PREV_TAG=$(shell git describe --tags $(git rev-list --parents -n 1 HEAD) | tail -n 1)

# assuming the use of Docker hub, these constants need not be changed
CONTAINER=$(DOCKER_ACCOUNT)/$(CONTAINER_NAME):$(TAG)

.PHONY: setup
setup: initdb
	@# check installation of `entr`, a CLI tool for running files on change
	@which concurrently > /dev/null; \
	if [[ $$? -ne 0 ]]; then \
		echo "fatal: must install entr. See https://eradman.com/entrproject/"; \
		exit 1; \
	fi


.PHONY: initdb
initdb:
	@if [[ '$(shell docker ps | grep postgres)' != "" ]]; then \
		echo "a postgres container is already running. skipping initdb"; \
	else \
		echo "initializing database..."; \
		docker run -d \
			-e POSTGRES_USER=app \
			-e POSTGRES_PASSWORD=app \
			-e POSTGRES_DB=app \
			-p 5432:5432 \
			postgres:14; \
		echo "database is starting up"; \
	fi;


.PHONY: develop
develop: setup
	find src | entr -r cargo run


.PHONY: deploy
deploy: setup
ifdef CI
	terraform init -input=false
endif
	terraform apply -auto-approve


.PHONY:
pre-docker-build:
	cargo build --release
	cp ../target/release/backend .


.PHONY: push
push: pre-docker-build
	docker buildx build --pull --platform linux/amd64 --push -t $(CONTAINER) .


.PHONY: check
check:
	cargo test
