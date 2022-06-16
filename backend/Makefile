SHELL=/bin/bash

DOCKER_ACCOUNT=jdevries3133
CONTAINER_NAME=nyc_bike_backend
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


.PHONY: setup-ci
setup-ci:
	terraform init -input=false


.PHONY: develop
develop: initdb
	find src | entr -r cargo run


.PHONY: deploy
deploy: push
	terraform apply -auto-approve


.PHONY: pre-container-build
pre-container-build:
ifeq ($(shell arch), "x86_64")
	cargo build --release
	cp ../target/release/backend .
else
	cargo build --release --target x86_64-unknown-linux-gnu
	cp ../target/x86_64-unknown-linux-gnu/release/backend .
endif


.PHONY: push
push: pre-container-build
	docker buildx build --platform linux/x86_64 --push -t $(CONTAINER) .


.PHONY: build
build: pre-container-build
	docker buildx build --platform linux/x86_64 --load -t $(CONTAINER) .
