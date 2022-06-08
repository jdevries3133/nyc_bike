SHELL=/bin/bash

DOCKER_ACCOUNT=jdevries3133
CONTAINER_NAME=rust_template_backend
TAG?=$(shell git describe --tags)
CONTAINER=$(DOCKER_ACCOUNT)/$(CONTAINER_NAME):$(TAG)


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


.PHONY: pre-container-build
pre-container-build:
	cargo build --release
	cp ../target/release/backend .


.PHONY: push
push: pre-container-build
	docker buildx build --pull --platform linux/amd64 --push -t $(CONTAINER) .


.PHONY: check
check:
	cargo test
